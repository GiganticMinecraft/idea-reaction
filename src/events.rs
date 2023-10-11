use crate::config::ENV_CONFIG;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::prelude::{ChannelId, GuildId, Webhook};
use serenity::prelude::EventHandler;
use tracing::log::info;

pub struct EvHandler;

#[async_trait]
impl EventHandler for EvHandler {
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
