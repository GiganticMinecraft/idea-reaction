#![deny(clippy::all)]

use anyhow::Context;
use handler::Handler;
use serenity::{Client, all::GatewayIntents};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod actions;
mod handler;
mod parsers;
mod redmine;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IdeaReactionEnv {
    #[serde(default)]
    pub env_name: String, // default: development

    pub discord_api_token: String,
    pub redmine_api_key: Option<String>,

    #[serde(default)]
    pub redmine_url: String, // default: https://localhost:8080

    pub target_channel_id: u64,
    pub target_guild_id: u64,
    pub target_webhook_id: u64,
}

impl Default for IdeaReactionEnv {
    fn default() -> Self {
        Self {
            env_name: "development".to_string(),
            discord_api_token: "".to_string(),
            redmine_api_key: None,
            redmine_url: "https://localhost:8080".to_string(),
            target_channel_id: 0,
            target_guild_id: 0,
            target_webhook_id: 0,
        }
    }
}

pub fn envs() -> &'static IdeaReactionEnv {
    static CACHE: std::sync::OnceLock<IdeaReactionEnv> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| envy::from_env().expect("Failed to load environment variables"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "idea_reaction=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    if let Err(why) = dotenvy::dotenv() {
        tracing::warn!("Failed to load .env file: {:?}", why);
    }

    let envs = envs();

    let _guard = if &envs.env_name == "production" {
        let client = sentry::init((
            "https://30bda4f81272fbe087c998d83ee5e960@sentry.onp.admin.seichi.click//6",
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                ..Default::default()
            },
        ));
        sentry::configure_scope(|s| s.set_level(Some(sentry::Level::Warning)));
        Some(client)
    } else {
        None
    };

    let intents =
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&envs.discord_api_token, intents)
        .event_handler(Handler)
        .await
        .context("Failed to create Discord client")?;

    if let Err(why) = client.start().await {
        println!("Failed run discord client: {:?}", why);
    }

    Ok(())
}
