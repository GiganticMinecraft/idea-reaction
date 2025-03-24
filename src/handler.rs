use crate::actions::IdeaReactionAction;
use crate::parsers::{parse_embed, parse_env_ids};
use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::EventHandler;

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
                }
                None => return,
            }
        } else {
            return;
        };

        let Some(first_embed) = message.embeds.first() else {
            return;
        };

        let embed = match parse_embed(first_embed) {
            Ok(embed) => embed,
            Err(why) => {
                tracing::error!("Failed to parse embed: {:?}", why);
                return;
            }
        };

        let action = IdeaReactionAction::builder()
            .ctx(ctx)
            .message(message)
            .issue_title(embed.title)
            .issue_number(embed.issue_number)
            .build();
        if let Err(why) = action.run().await {
            tracing::info!("Failed to run action: {:?}", why);
        }
    }

    #[tracing::instrument(skip_all)]
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(
            ActivityData::custom(format!("Running v{}", env!("CARGO_PKG_VERSION"))).into(),
        );
        tracing::info!("{} is connected!", ready.user.name);
    }
}
