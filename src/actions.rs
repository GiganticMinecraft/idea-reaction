use anyhow::Context as _;
use serenity::all::ReactionType;
use serenity::client::Context;
use serenity::model::channel::Message;
use typed_builder::TypedBuilder;

const REACTION_EMOJIS: [&str; 2] = ["👍", "👎"];

#[derive(TypedBuilder)]
pub struct IdeaReactionAction {
    pub ctx: Context,
    pub message: Message,
}

impl IdeaReactionAction {
    pub async fn run(&self) -> anyhow::Result<()> {
        for r in REACTION_EMOJIS {
            self.message
                .react(&self.ctx.http, ReactionType::Unicode(r.to_string()))
                .await
                .context("Failed to react")?;
        }
        Ok(())
    }
}
