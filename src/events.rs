use serenity::async_trait;
use serenity::client::Context;
use serenity::gateway::ActivityData;
use serenity::model::gateway::Ready;
use serenity::model::id::WebhookId;
use serenity::model::prelude::{ChannelId, GuildId};
use serenity::prelude::EventHandler;

use crate::envs;

pub struct EvHandler;

#[async_trait]
impl EventHandler for EvHandler {
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
