use async_trait::async_trait;

#[derive(Debug)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}

#[derive(Debug)]
pub struct IssueCreatedEvent {
    pub id: usize,
    pub title: String,
    pub body: String,
    pub user: String,
}

#[derive(Debug)]
pub enum UpdatedPart {
    Title { from: String, to: String },
    Body { from: String, to: String },
}

#[derive(Debug)]
pub struct IssueUpdatedEvent {
    pub id: usize,
    pub updated_part: UpdatedPart,
    pub user: String,
}

#[derive(Debug)]
pub struct PullRequestCreatedEvent {
    pub id: usize,
    pub title: String,
    pub body: String,
    pub user: String,
}

#[derive(Debug)]
pub struct PullRequestUpdatedEvent {
    pub id: usize,
    pub updated_part: UpdatedPart,
    pub user: String,
}

#[derive(Debug)]
pub enum CommentTarget {
    Issue(usize),
    PullRequest(usize),
}

impl CommentTarget {
    pub fn id(&self) -> usize {
        match self {
            CommentTarget::Issue(x) => *x,
            CommentTarget::PullRequest(x) => *x,
        }
    }
}

#[derive(Debug)]
pub struct CommentCreatedEvent {
    pub id: usize,
    pub target: CommentTarget,
    pub body: String,
}

#[derive(Debug)]
pub struct CommentUpdatedEvent {
    pub id: usize,
    pub target: CommentTarget,
    pub from: String,
    pub to: String,
}

#[async_trait]
pub trait Bot: Send + Sync {
    async fn on_issue_created(&self, _repo: Repository, _event: IssueCreatedEvent) {}

    async fn on_issue_updated(&self, _repo: Repository, _event: IssueUpdatedEvent) {}

    async fn on_issue_closed(&self, _repo: Repository, _issue_id: usize) {}

    async fn on_pull_request_created(&self, _repo: Repository, _event: PullRequestCreatedEvent) {}

    async fn on_pull_request_updated(&self, _repo: Repository, _event: PullRequestUpdatedEvent) {}

    async fn on_pull_request_closed(&self, _repo: Repository, _pull_request_id: usize) {}

    async fn on_comment_created(&self, _repo: Repository, _event: CommentCreatedEvent) {}

    async fn on_comment_updated(&self, _repo: Repository, _event: CommentUpdatedEvent) {}

    async fn on_comment_deleted(&self, _repo: Repository, _comment_id: usize) {}
}

pub struct Dispatcher<T: Bot> {
    core: T,
}

impl<T: Bot> Dispatcher<T> {
    pub fn new(core: T) -> Self {
        Dispatcher { core }
    }

    pub async fn dispatch_event(&self, event: serde_json::Value) {
        let event_name: &str = event["event_name"].as_str().unwrap();
        match event_name {
            "issues" if event["event"]["issue"].get("pull_request").is_some() => {
                self.dispatch_pull_request_event(event).await
            }
            "pull_request" => self.dispatch_pull_request_event(event).await,
            "issues" => self.dispatch_issues_event(event).await,
            "issue_comment" => self.dispatch_issue_comment_event(event).await,
            _ => unimplemented!(),
        }
    }

    async fn dispatch_issues_event(&self, event: serde_json::Value) {
        let event_action: &str = event["event"]["action"].as_str().unwrap();
        let repo = Repository {
            owner: event["event"]["repository"]["owner"]["login"]
                .as_str()
                .unwrap()
                .to_string(),
            name: event["repository"]
                .as_str()
                .unwrap()
                .split('/')
                .nth(1)
                .unwrap()
                .to_string(),
        };
        match event_action {
            "opened" => {
                let inner_event = IssueCreatedEvent {
                    id: event["event"]["issue"]["number"].as_u64().unwrap() as _,
                    title: event["event"]["issue"]["title"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    body: event["event"]["issue"]["body"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    user: event["event"]["issue"]["user"]["login"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                };
                self.core.on_issue_created(repo, inner_event).await;
            }
            "closed" => {
                let id = event["event"]["issue"]["number"].as_u64().unwrap() as usize;
                self.core.on_issue_closed(repo, id).await;
            }
            "updated" => {
                let updated_part = if event["event"]["changes"].get("body").is_some() {
                    UpdatedPart::Body {
                        from: event["event"]["changes"]["body"]["from"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                        to: event["event"]["issue"]["body"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    }
                } else {
                    UpdatedPart::Title {
                        from: event["event"]["changed"]["body"]["from"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                        to: event["event"]["issue"]["title"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    }
                };
                let inner_event = IssueUpdatedEvent {
                    id: event["event"]["issue"]["number"].as_u64().unwrap() as usize,
                    updated_part,
                    user: event["event"]["issue"]["user"]["login"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                };
                self.core.on_issue_updated(repo, inner_event).await;
            }
            _ => unimplemented!(),
        }
    }

    async fn dispatch_pull_request_event(&self, event: serde_json::Value) {
        let repo = Repository {
            owner: event["event"]["repository"]["owner"]["login"]
                .as_str()
                .unwrap()
                .to_string(),
            name: event["repository"]
                .as_str()
                .unwrap()
                .split('/')
                .nth(1)
                .unwrap()
                .to_string(),
        };
        let event_action: &str = event["event"]["action"].as_str().unwrap();
        match event_action {
            "opened" => {
                let inner_event = PullRequestCreatedEvent {
                    id: event["event"]["pull_request"]["number"].as_u64().unwrap() as _,
                    title: event["event"]["pull_request"]["title"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    body: event["event"]["pull_request"]["body"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    user: event["event"]["pull_request"]["user"]["login"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                };
                self.core.on_pull_request_created(repo, inner_event).await;
            }
            "closed" => {
                let id = event["event"]["issue"]["number"].as_u64().unwrap() as usize;
                self.core.on_pull_request_closed(repo, id).await;
            }
            "edited" => {
                let updated_part = if event["event"]["changes"].get("body").is_some() {
                    UpdatedPart::Body {
                        from: event["event"]["changes"]["body"]["from"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                        to: event["event"]["pull_request"]["body"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    }
                } else {
                    UpdatedPart::Title {
                        from: event["event"]["changed"]["body"]["from"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                        to: event["event"]["pull_request"]["title"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    }
                };
                let inner_event = PullRequestUpdatedEvent {
                    id: event["event"]["issue"]["number"].as_u64().unwrap() as usize,
                    updated_part,
                    user: event["event"]["issue"]["user"]["login"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                };
                self.core.on_pull_request_updated(repo, inner_event).await;
            }
            _ => unimplemented!(),
        }
    }

    async fn dispatch_issue_comment_event(&self, event: serde_json::Value) {
        let repo = Repository {
            owner: event["event"]["repository"]["owner"]["login"]
                .as_str()
                .unwrap()
                .to_string(),
            name: event["repository"]
                .as_str()
                .unwrap()
                .split('/')
                .nth(1)
                .unwrap()
                .to_string(),
        };
        let event_action: &str = event["event"]["action"].as_str().unwrap();
        let target = if event["event"]["issue"].get("pull_request").is_some() {
            CommentTarget::PullRequest(event["event"]["issue"]["number"].as_u64().unwrap() as _)
        } else {
            CommentTarget::Issue(event["event"]["issue"]["number"].as_u64().unwrap() as _)
        };
        match event_action {
            "created" => {
                let inner_event = CommentCreatedEvent {
                    id: event["event"]["comment"]["id"].as_u64().unwrap() as usize,
                    target,
                    body: event["event"]["comment"]["body"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                };
                self.core.on_comment_created(repo, inner_event).await;
            }
            "deleted" => {
                let id = event["event"]["comment"]["id"].as_u64().unwrap() as usize;
                self.core.on_comment_deleted(repo, id).await;
            }
            "edited" => {
                let inner_event = CommentUpdatedEvent {
                    id: event["event"]["comment"]["id"].as_u64().unwrap() as usize,
                    target,
                    from: event["event"]["changes"]["from"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    to: event["event"]["comment"]["body"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                };
                self.core.on_comment_updated(repo, inner_event).await;
            }
            _ => unimplemented!(),
        }
    }
}
