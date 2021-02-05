use async_trait::async_trait;
use baipiao_bot_rust::{
    Bot, CommentCreatedEvent, Dispatcher, IssueCreatedEvent, PullRequestCreatedEvent, Repository,
};
use octocrab::{Octocrab, OctocrabBuilder};
use std::env;
use std::io::Read;

struct CommentBot {
    github_client: Octocrab,
}

impl CommentBot {
    fn new(token: String) -> Self {
        Self {
            github_client: OctocrabBuilder::new()
                .personal_token(token)
                .build()
                .unwrap(),
        }
    }

    async fn comment(&self, repo: Repository, issue_id: usize, content: &str) {
        self.github_client
            .issues(repo.owner, repo.name)
            .create_comment(issue_id as u64, content)
            .await
            .unwrap();
    }
}

#[async_trait]
impl Bot for CommentBot {
    async fn on_issue_created(&self, repo: Repository, event: IssueCreatedEvent) {
        self.comment(
            repo,
            event.id,
            "我 劝 开发者耗子为之，耗耗反思，不要再犯这样的错误，小错误啊",
        )
            .await
    }

    async fn on_issue_closed(&self, repo: Repository, issue_id: usize) {
        self.comment(repo, issue_id, "我啪的一下就修好了，很快啊")
            .await
    }

    async fn on_pull_request_created(&self, repo: Repository, event: PullRequestCreatedEvent) {
        self.comment(
            repo,
            event.id,
            &format!(
                "按 传统发pr的点到为止 @{} 已经赢了，如果这个pr合进去，一个pr就把问题给解决了",
                event.user
            ),
        )
            .await
    }

    async fn on_comment_created(&self, repo: Repository, event: CommentCreatedEvent) {
        if event.body.contains("@baipiao-bot") {
            if event.body.to_lowercase().contains("nsml") {
                self.comment(repo,
                             event.target.id(),
                             &format!(
                                 "@{} NMYSL!",
                                 event.commenter_user_login
                             ),
                )
            } else {
                self.comment(repo, event.target.id(), &format!("发生甚么事了"))
            }
        }
            .await
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("BAIPIAO_BOT_TOKEN").unwrap();
    let bot = CommentBot::new(token);
    let dispatcher = Dispatcher::new(bot);
    let content = env::var("JSON").unwrap();
    let input: serde_json::Value = serde_json::from_str(&content).unwrap();
    dispatcher.dispatch_event(input).await;
}
