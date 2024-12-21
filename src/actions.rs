use crate::redmine::actions::RedmineAction;
use serenity::all::{AutoArchiveDuration, ChannelType, ReactionType};
use serenity::builder::CreateThread;
use serenity::client::Context;
use serenity::model::channel::Message;
use typed_builder::TypedBuilder;

const REACTION_EMOJIS: [&str; 3] = ["👍", "👎", "🤔"];

#[derive(TypedBuilder)]
pub struct IdeaReactionAction {
    pub ctx: Context,
    pub message: Message,
    pub issue_title: String,
    pub issue_number: u16,
}

#[derive(thiserror::Error, Debug)]
pub enum IdeaReactionActionError {
    #[error("Failed to reaction. (Reason: {0})")]
    FailedToReaction(String),
    #[error("Failed to create thread. (Reason: {0})")]
    FailedToCreateThread(String),
    #[error("Failed to send request to Redmine. (Reason: {0})")]
    FailedToSendRedmineComment(String),
}

impl IdeaReactionAction {
    pub async fn run(&self) -> anyhow::Result<(), IdeaReactionActionError> {
        let envs = crate::envs();

        // リアクションを付与
        for r in REACTION_EMOJIS {
            if let Err(why) = self
                .message
                .react(&self.ctx.http, ReactionType::Unicode(r.to_string()))
                .await
            {
                return Err(IdeaReactionActionError::FailedToReaction(why.to_string()));
            }
        }

        // スレッドを作成
        let t = match self
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
        {
            Ok(t) => t,
            Err(why) => {
                return Err(IdeaReactionActionError::FailedToCreateThread(
                    why.to_string(),
                ));
            }
        };

        // スレッドのURLを Redmine にコメントする. (Serenity はスレッドの URL を取得するメソッドがない)
        let content = format!(
            "Thread: https://discord.com/channels/{}/{}",
            envs.target_guild_id, t.id
        );
        if let Err(why) = RedmineAction::run(self.issue_number, content).await {
            return Err(IdeaReactionActionError::FailedToSendRedmineComment(
                why.to_string(),
            ));
        }

        Ok(())
    }
}
