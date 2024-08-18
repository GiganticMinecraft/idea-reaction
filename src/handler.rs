use serenity::async_trait;
use serenity::client::Context;
use serenity::gateway::ActivityData;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::WebhookId;
use serenity::model::prelude::{ChannelId, GuildId};
use serenity::prelude::EventHandler;
use crate::actions::action_to_idea;
use crate::envs;
use crate::parsers::{parse_embed, parse_env_ids};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[tracing::instrument(skip_all)]
    async fn message(&self, ctx: Context, message: Message) {
        let target_ids = parse_env_ids().expect("Failed to parse environment IDs");

        // メッセージが idea-reaction 監視対象の Webhook であるかを確認
        if message.channel_id == target_ids.channel {
            match message.webhook_id {
                Some(wb) => {
                    if wb != target_ids.webhook {
                        return;
                    }
                    wb
                },
                None => return,
            }
        } else {
            return;
        };

        if let Err(why) = parse_embed(message.embeds.first()) {
            tracing::info!("Failed to parse embed: {:?}", why);
            return;
        };

        if let Err(why) = action_to_idea(&ctx, &message).await {
            tracing::info!("Failed to react: {:?}", why);
        }
    }

    #[tracing::instrument(skip_all)]
    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);

        let version = env!("CARGO_PKG_VERSION");
        let envs = envs();
        let guild_id = GuildId::new(envs.target_guild_id);
        let channels = guild_id.channels(&ctx.http).await.unwrap();

        match channels.get(&ChannelId::new(envs.target_channel_id)) {
            Some(ch) => {
                let webhooks = ch.webhooks(&ctx.http).await.unwrap();
                let target_wb = webhooks
                    .iter()
                    .find(|wb| wb.id == WebhookId::new(envs.target_webhook_id));

                match target_wb {
                    Some(w) => {
                        tracing::info!("Watching webhook: {}", w.name.clone().unwrap());
                    }
                    None => {
                        panic!("Target webhook is not found!");
                    }
                }

                ctx.set_activity(Some(ActivityData::watching(format!(
                    "v{} - Watching, #{}",
                    version, &ch.name
                ))))
            }
            None => {
                panic!("Target channel is not found!");
            }
        }
    }
}
