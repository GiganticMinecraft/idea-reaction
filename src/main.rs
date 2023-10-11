use crate::client::discord::create_discord_client;
use crate::config::load_config;
use dotenvy::dotenv;
use std::env;

mod client;
mod config;
mod events;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().compact().init();

    let token = env::var("DISCORD_API_TOKEN").expect("Expected a token in the environment");
    let mut client = create_discord_client(&token).await?;

    load_config()?;

    if let Err(why) = client.start().await {
        println!("Failed run discord client: {:?}", why);
    }

    Ok(())
}
