use anyhow::Context as _;
use serenity::all::ReactionType;
use serenity::client::Context;
use serenity::model::channel::Message;

const REACTION_EMOJIS: [&str; 2] = ["👍", "👎"];

pub async fn action_to_idea(ctx: &Context, message: &Message) -> anyhow::Result<()> {
    for r in REACTION_EMOJIS {
        message
            .react(&ctx.http, ReactionType::Unicode(r.to_string()))
            .await.context("Failed to react")?;
    }
    Ok(())
}
