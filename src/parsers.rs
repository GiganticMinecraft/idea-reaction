use serenity::all::{ChannelId, Embed, WebhookId};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct IdeaReactionEnvIDs {
    pub channel: ChannelId,
    pub webhook: WebhookId,
}

pub fn parse_env_ids() -> anyhow::Result<IdeaReactionEnvIDs> {
    let envs = crate::envs();

    let result = IdeaReactionEnvIDs::builder()
        .channel(ChannelId::new(envs.target_channel_id))
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
        }
        None => anyhow::bail!("Failed to parse embed"),
    }
}

pub fn parse_issue_number(title: String) -> u16 {
    let re = regex::Regex::new(r"#(\d+)").unwrap();
    re.captures(&title)
        .map(|caps| caps.get(1).unwrap().as_str().parse::<u16>().ok().unwrap())
        .unwrap()
}
