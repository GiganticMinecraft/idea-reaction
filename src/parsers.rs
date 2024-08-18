use serenity::all::{ChannelId, Embed, GuildId, WebhookId};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct IdeaReactionEnvIDs  {
    pub channel: ChannelId,
    pub guild: GuildId,
    pub webhook: WebhookId,
}

pub fn parse_env_ids() -> anyhow::Result<IdeaReactionEnvIDs> {
    let envs = crate::envs();

    let result = IdeaReactionEnvIDs::builder()
        .channel(ChannelId::new(envs.target_channel_id))
        .guild(GuildId::new(envs.target_guild_id))
        .webhook(WebhookId::new(envs.target_webhook_id))
        .build();

    Ok(result)
}

pub fn parse_embed(embed: Option<&Embed>) -> anyhow::Result<Embed> {
    match embed {
        Some(e) => {
            // FIXME: なぜか所有権が譲渡されない...
            if !e.clone().title.unwrap().contains("[New issue]") {
                anyhow::bail!("Embed title is not '[New issue]'. skipping...");
            }
            Ok(e.clone())
        },
        None => anyhow::bail!("Failed to parse embed"),
    }
}
