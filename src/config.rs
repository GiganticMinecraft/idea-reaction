use anyhow::Context;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use tracing::log::info;

#[derive(Default, Deserialize, Debug)]
pub struct IdeaReactionConfig {
    pub reactions: Vec<String>,
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

    info!("Loaded config: {:?}", config.reactions);
    Ok(config)
}
