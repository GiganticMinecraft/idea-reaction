use crate::redmine::RedmineAction;
use anyhow::Context as _;
use serenity::all::{AutoArchiveDuration, ChannelType, ReactionType};
use serenity::builder::CreateThread;
use serenity::client::Context;
use serenity::model::channel::Message;
use typed_builder::TypedBuilder;

#[cfg(feature = "experiments_thinking_emoji")]
const REACTION_EMOJIS: [&str; 3] = ["👍", "👎", "🤔"];

#[cfg(not(feature = "experiments_thinking_emoji"))]
const REACTION_EMOJIS: [&str; 2] = ["👍", "👎"];

#[derive(TypedBuilder)]
pub struct IdeaReactionAction {
    pub ctx: Context,
    pub message: Message,
    pub issue_title: String,
    pub issue_number: u16,
}

impl IdeaReactionAction {
    pub async fn run(&self) -> anyhow::Result<()> {
        let envs = crate::envs();

        // リアクションを付与
        for r in REACTION_EMOJIS {
            self.message
                .react(&self.ctx.http, ReactionType::Unicode(r.to_string()))
                .await
                .context("Failed to reaction.")?;
        }

        // スレッドを作成
        let t = self
            .message
            .channel_id
            .create_thread_from_message(&self.ctx.http, &self.message.id, {
                CreateThread::new(
                    self.issue_title
                        // FIXME: パースを正規表現を使って改善の余地あり
                        .replace("[New issue] アイデア提案用プロジェクト - アイデア提案 ", ""),
                )
                .kind(ChannelType::PublicThread)
                .auto_archive_duration(AutoArchiveDuration::OneWeek)
            })
            .await
            .context("Failed to create thread.")?;

        // スレッドのURLを Redmine にコメントする. (Serenity はスレッドの URL を取得するメソッドがない)
        let content = format!(
            "Thread: https://discord.com/channels/{}/{}",
            envs.target_guild_id, t.id
        );
        RedmineAction::run(self.issue_number, content).await?;

        Ok(())
    }
}
