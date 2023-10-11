use crate::events::EvHandler;
use anyhow::Context;
use serenity::prelude::GatewayIntents;
use serenity::Client;

pub async fn create_discord_client(token: &str) -> anyhow::Result<Client> {
    let intents =
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(token, intents)
        .event_handler(EvHandler)
        .await
        .context("Failed to create Discord client")?;

    Ok(client)
}
