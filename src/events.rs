use serenity::async_trait;
use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::prelude::EventHandler;
use tracing::log::info;

pub struct EvHandler;

#[async_trait]
impl EventHandler for EvHandler {
    async fn ready(&self, ctx: Context, bot: Ready) {
        info!("{} is connected!", bot.user.name);
    }
}
