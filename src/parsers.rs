use serenity::all::{ChannelId, WebhookId};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct IdeaReactionEnvIDs {
    pub channel: ChannelId,
    pub webhook: WebhookId,
}

// https://github.com/kory33/redmine_discord/blob/master/lib/redmine_discord/embed_objects/wiki_embeds.rb
#[derive(TypedBuilder)]
pub struct IdeaEmbed {
    pub title: String,
    pub issue_number: u16,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseEnvIDsError {
    #[error("Embed is not new issue embed.")]
    NotNewIssueEmbed(),
    #[error("Failed to parse embed. (Reason: {0})")]
    FailedToParseEmbed(String),
    #[error("Failed to parse issue number. (Reason: {0})")]
    FailedToParseIssueNumber(String),
}

pub fn parse_env_ids() -> anyhow::Result<IdeaReactionEnvIDs> {
    let envs = crate::envs();

    let result = IdeaReactionEnvIDs::builder()
        .channel(ChannelId::new(envs.target_channel_id))
        .webhook(WebhookId::new(envs.target_webhook_id))
        .build();

    Ok(result)
}

pub fn parse_embed(embed: &serenity::all::Embed) -> anyhow::Result<IdeaEmbed, ParseEnvIDsError> {
    let title = match embed.title {
        Some(ref t) => {
            if !t.contains("[New issue]") {
                return Err(ParseEnvIDsError::NotNewIssueEmbed());
            }
            t.replace("[New issue]", "").trim().to_string();
            t
        }
        None => {
            return Err(ParseEnvIDsError::FailedToParseEmbed(
                "Title does not exist in embed.".to_string(),
            ))
        }
    };
    let issue_number = parse_issue_number(title)?;

    Ok(IdeaEmbed::builder()
        .title(title.clone())
        .issue_number(issue_number)
        .build())
}

fn parse_issue_number(title: &str) -> anyhow::Result<u16, ParseEnvIDsError> {
    let re = std::cell::LazyCell::new(|| regex::Regex::new(r"#(\d+)").unwrap());

    match re.captures(title) {
        Some(caps) => {
            let Some(matched) = caps.get(1) else {
                return Err(ParseEnvIDsError::FailedToParseIssueNumber(
                    "Failed to get capture group.".to_string(),
                ));
            };
            match matched.as_str().parse::<u16>() {
                Ok(num) => Ok(num),
                Err(why) => Err(ParseEnvIDsError::FailedToParseIssueNumber(why.to_string())),
            }
        }
        None => Err(ParseEnvIDsError::FailedToParseIssueNumber(
            "No captures found.".to_string(),
        )),
    }
}

#[cfg(test)]
mod parsers_test {
    use super::*;

    // `parse_issue_number()` が正しく Issue 番号をパースできるか
    #[test]
    fn test_parse_issue_number() {
        // Redmine に存在する実際の Issue からモックを作成
        let mock = [
            // https://redmine.seichi.click/issues/9
            "[New issue] アイデア提案用プロジェクト - アイデア提案 #9: 不定期イベントシステム",
            // https://redmine.seichi.click/issues/951
            "[New issue] アイデア提案用プロジェクト - アイデア提案 #951: 所有物の管理をUUIDにする名前を変えた",
            // https://redmine.seichi.click/issues/1527
            "[New issue] アイデア提案用プロジェクト - アイデア提案 #1527: 整地がメビウスなら建設にも成長する防具が",
            // https://redmine.seichi.click/issues/10925
            "[New issue] アイデア提案用プロジェクト - アイデア提案 #10925: 棒メニューに建築素材のみの購入メニュー"
        ];

        assert_eq!(parse_issue_number(mock[0]).unwrap(), 9);
        assert_eq!(parse_issue_number(mock[1]).unwrap(), 951);
        assert_eq!(parse_issue_number(mock[2]).unwrap(), 1527);
        assert_eq!(parse_issue_number(mock[3]).unwrap(), 10925);
    }

    // `parse_issue_number()` が Issue 番号がないタイトルのパースを正しく失敗できるか
    #[test]
    fn test_parse_issue_number_no_capture() {
        let mock = "[New issue] アイデア提案用プロジェクト - アイデア提案: 不定期イベントシステム";
        let result = parse_issue_number(mock);

        assert!(result.is_err());
    }
}
