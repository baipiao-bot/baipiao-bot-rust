use async_trait::async_trait;
use baipiao_bot_rust::{
    Bot, CommentCreatedEvent, CommentUpdatedEvent, Dispatcher, IssueCreatedEvent,
    IssueUpdatedEvent, PullRequestCreatedEvent, PullRequestUpdatedEvent, Repository,
};
use std::io::Read;

struct EchoBot;

#[async_trait]
impl Bot for EchoBot {
    async fn on_issue_created(&self, repo: Repository, event: IssueCreatedEvent) {
        println!("on_issue_created: {:?}, {:?}", repo, event)
    }

    async fn on_issue_updated(&self, repo: Repository, event: IssueUpdatedEvent) {
        println!("on_issue_updated: {:?}, {:?}", repo, event)
    }

    async fn on_issue_closed(&self, repo: Repository, issue_id: usize) {
        println!("on_issue_closed: {:?}, {:?}", repo, issue_id)
    }

    async fn on_pull_request_created(&self, repo: Repository, event: PullRequestCreatedEvent) {
        println!("on_pull_request_created: {:?}, {:?}", repo, event)
    }

    async fn on_pull_request_updated(&self, repo: Repository, event: PullRequestUpdatedEvent) {
        println!("on_pull_request_updated: {:?}, {:?}", repo, event)
    }

    async fn on_pull_request_closed(&self, repo: Repository, pull_request_id: usize) {
        println!("on_pull_request_closed: {:?}, {:?}", repo, pull_request_id)
    }

    async fn on_comment_created(&self, repo: Repository, event: CommentCreatedEvent) {
        println!("on_comment_created: {:?}, {:?}", repo, event)
    }

    async fn on_comment_updated(&self, repo: Repository, event: CommentUpdatedEvent) {
        println!("on_comment_updated: {:?}, {:?}", repo, event)
    }

    async fn on_comment_deleted(&self, repo: Repository, comment_id: usize) {
        println!("on_comment_deleted: {:?}, {:?}", repo, comment_id)
    }
}

#[tokio::main]
async fn main() {
    let bot = EchoBot;
    let dispatcher = Dispatcher::new(bot);
    let mut content = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut content)
        .unwrap();
    let input: serde_json::Value = serde_json::from_str(&content).unwrap();
    dispatcher.dispatch_event(input).await;
}
