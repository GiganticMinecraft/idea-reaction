use anyhow::Context;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use tracing::log::info;

#[derive(Default, Deserialize, Debug)]
pub struct IdeaReactionConfig {
    pub reactions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdeaReactionEnv {
    pub discord_api_token: String,
    pub target_channel_id: u64,
    pub target_guild_id: u64,
    pub target_webhook_id: u64,
}

pub fn load_config() -> anyhow::Result<IdeaReactionConfig> {
    let config: IdeaReactionConfig = serde_yaml::from_reader(BufReader::new(
        File::open("./assets/config.yaml").context("Failed to open config file")?,
    ))
    .context("Failed to parse config file")?;

    // TODO: このチェックを厳格化する
    if config.reactions.is_empty() {
        panic!("No reactions found in config file");
    }

    Ok(config)
}

pub static ENV_CONFIG: OnceCell<IdeaReactionEnv> = OnceCell::new();
