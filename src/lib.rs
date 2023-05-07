pub mod server;
pub mod worker;
use serde::{Deserialize, Serialize};

/// An execution Step in a [`Job`]
#[derive(Debug, Deserialize, Serialize)]
pub struct Step {
    name: Option<String>,
    command: String
}

/// The result of executing a [`Step`]
#[derive(Debug)]
pub enum StepStatus {
    Failed,
    Succeded
}

/// A set of [`Step`] with a single objective
#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    name: String,
    steps: Vec<Step>,
    image_name: String,
}

/// The status of a given [`Job`]
#[derive(Debug, Deserialize, Serialize)]
pub enum JobStatus {
    Started,
    Running,
    Paused,
    Failed
}

/// A Build
#[derive(Debug, Deserialize, Serialize)]
pub struct Build {
    id: String,
    jobs: Vec<Job>,
    commit: CommitInfo
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BuildStatus {
    Pending,
    Started,
    Working,
    Failed,
    Timeout
}

#[derive(Debug,Deserialize, Serialize)]
pub struct BuildUpdate {
    build_id: String,
    status: BuildStatus
}

/// Identifies commit information.
#[derive(Debug, Deserialize, Serialize)]
pub struct CommitInfo {
    id: String,
    message: String,
    url: String,
    author: ShortUserInfo,
    committer: ShortUserInfo,
    timestamp: String
}

/// Long user identification
#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    id: String,
    login: String,
    full_name: String,
    email: String,
    avatar_url: String,
    username: String
}

/// Short user identification
#[derive(Debug, Deserialize, Serialize)]
pub struct ShortUserInfo {
    name: String,
    email: String,
    username: String
}

/// Identifies repository data.
#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryInfo {
    id: String,
    owner: UserInfo,
    name: String,
    full_name: String,
    description: Option<String>,
    private: bool,
    fork: bool,
    html_url: String,
    ssh_url: String,
    clone_url: String,
    website: Option<String>,
    stars_count: u32,
    forks_count: u32,
    watchers_count: u32,
    open_issues_count: u32,
    default_branch: String,
    created_at: String,
    updated_at: String
}

/// Gitea commit webhook
#[derive(Debug, Deserialize, Serialize)]
pub struct WebHook {
    secrets: Option<String>,
    refs: String,
    before: String,
    after: String,
    compare_url: String,
    commits: Vec<CommitInfo>,
    repository: RepositoryInfo,
    pusher: UserInfo,
    sender: UserInfo
}
