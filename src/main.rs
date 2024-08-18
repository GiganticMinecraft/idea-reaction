use anyhow::Context;
use handler::Handler;
use serenity::{all::GatewayIntents, Client};

mod actions;
mod handler;
mod parsers;
mod redmine;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IdeaReactionEnv {
    pub discord_api_token: String,
    pub redmine_api_key: String,
    pub redmine_url: String,
    pub target_channel_id: u64,
    pub target_guild_id: u64,
    pub target_webhook_id: u64,
}

pub fn envs() -> &'static IdeaReactionEnv {
    static CACHE: std::sync::OnceLock<IdeaReactionEnv> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| envy::from_env().expect("Failed to load environment variables"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(why) = dotenvy::dotenv() {
        tracing::warn!("Failed to load .env file: {:?}", why);
    }

    let envs = envs();

    tracing_subscriber::fmt().compact().init();

    let intents =
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&envs.discord_api_token, intents)
        .event_handler(Handler)
        .await
        .context("Failed to create Discord client")?;

    // https://github.com/GiganticMinecraft/idea-reaction/issues/107
    if cfg!(feature = "experiments_thinking_emoji") {
        tracing::info!("[experiments_thinking_emoji] が有効になっています. 🤔");
    }

    if let Err(why) = client.start().await {
        println!("Failed run discord client: {:?}", why);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parsers::parse_issue_number;

    #[test]
    fn test_parse_issue_number() {
        let mocks: [String; 2] = ["#13000".parse().unwrap(), "#1".parse().unwrap()];
        mocks.iter().for_each(|m| {
            let r = parse_issue_number(m.to_string());
            assert_eq!(r, m[1..].parse::<u16>().unwrap());
        });
    }
}
