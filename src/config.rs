use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

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

pub static ENV_CONFIG: OnceCell<IdeaReactionEnv> = OnceCell::new();
