use anyhow::Context as _;
use serenity::all::{AutoArchiveDuration, ChannelType, ReactionType};
use serenity::builder::CreateThread;
use serenity::client::Context;
use serenity::model::channel::Message;
use typed_builder::TypedBuilder;

const REACTION_EMOJIS: [&str; 2] = ["👍", "👎"];

#[derive(TypedBuilder)]
pub struct IdeaReactionAction {
    pub ctx: Context,
    pub message: Message,
    pub issue_title: String,
}

impl IdeaReactionAction {
    pub async fn run(&self) -> anyhow::Result<()> {
        // リアクションを付与
        for r in REACTION_EMOJIS {
            self.message
                .react(&self.ctx.http, ReactionType::Unicode(r.to_string()))
                .await
                .context("Failed to reaction.")?;
        }

        // スレッドを作成
        self.message
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

        Ok(())
    }
}
