use crate::client::discord::create_discord_client;
use crate::config::{load_config, IdeaReactionEnv, ENV_CONFIG};
use dotenvy::dotenv;
use std::env;

mod client;
mod config;
mod events;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().compact().init();

    ENV_CONFIG
        .set(envy::from_env::<IdeaReactionEnv>().expect("Failed to load environment variables"))
        .unwrap();

    let mut client = create_discord_client(&ENV_CONFIG.get().unwrap().discord_api_token).await?;

    load_config()?;

    if let Err(why) = client.start().await {
        println!("Failed run discord client: {:?}", why);
    }

    Ok(())
}
