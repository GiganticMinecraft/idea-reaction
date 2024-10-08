use crate::actions::IdeaReactionAction;
use crate::parsers::{parse_embed, parse_env_ids, parse_issue_number};
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

        let embed = match parse_embed(message.embeds.first()) {
            Ok(embed) => embed,
            Err(why) => {
                tracing::error!("Failed to parse embed: {:?}", why);
                return;
            }
        };

        let action = IdeaReactionAction::builder()
            .ctx(ctx)
            .message(message)
            .issue_title(embed.title.clone().unwrap())
            .issue_number(parse_issue_number(embed.title.clone().unwrap()))
            .build();
        if let Err(why) = action.run().await {
            tracing::info!("Failed to run action: {:?}", why);
        }
    }

    #[tracing::instrument(skip_all)]
    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
}
