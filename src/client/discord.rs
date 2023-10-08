use crate::events::EvHandler;
use anyhow::Context;
use serenity::prelude::GatewayIntents;
use serenity::Client;

pub async fn create_discord_client(token: &str) -> anyhow::Result<Client> {
    let intents = GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::GUILD_MESSAGES;
    let client = Client::builder(token, intents)
        .event_handler(EvHandler)
        .await
        .context("Failed to create Discord client")?;

    Ok(client)
}
