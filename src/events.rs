use crate::config::{load_config, ENV_CONFIG};
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::WebhookId;
use serenity::model::prelude::{ChannelId, GuildId, ReactionType};
use serenity::prelude::EventHandler;
use tracing::log::info;

pub struct EvHandler;

#[async_trait]
impl EventHandler for EvHandler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.is_private() {
            return;
        }

        let env_config = ENV_CONFIG.get().unwrap();

        let channel_id = message.channel_id;
        if channel_id != ChannelId(env_config.target_channel_id) {
            return;
        }

        match message.webhook_id {
            Some(id) => {
                if id != WebhookId(env_config.target_webhook_id) {
                    return;
                }

                let embed = message.embeds.first().unwrap();
                let embed_title = embed.title.as_ref().unwrap();
                if !embed_title.contains("[New issue]") {
                    return;
                }

                let reactions = load_config().unwrap().reactions;
                for reaction in reactions {
                    if let Err(why) = message
                        .react(&ctx.http, ReactionType::Unicode(reaction))
                        .await
                    {
                        info!("Failed to react: {:?}", why);
                    }
                }

                info!("Reacted to message: {}", message.id);
            }
            None => {
                return;
            }
        }
    }
    async fn ready(&self, ctx: Context, bot: Ready) {
        let env_config = ENV_CONFIG.get().unwrap();
        info!(
            "{name}({id}) is connected!",
            name = bot.user.name,
            id = bot.user.id
        );

        let guild = GuildId(env_config.target_guild_id);
        let channel = guild.channels(&ctx.http).await.unwrap();

        match channel.get(&ChannelId(env_config.target_channel_id)) {
            Some(channel) => {
                info!("Watching channel: {}", channel.name);
                ctx.set_activity(Activity::watching(&channel.name)).await;

                let webhook = channel.webhooks(&ctx.http).await.unwrap();
                let target_webhook = webhook
                    .iter()
                    .find(|webhook| webhook.id == env_config.target_webhook_id);
                match target_webhook {
                    Some(w) => {
                        info!("Watching webhook: {}", w.name.clone().unwrap());
                    }
                    None => {
                        panic!("Target webhook is not found");
                    }
                }
            }
            None => {
                panic!("Target channel is not found");
            }
        }
    }
}
