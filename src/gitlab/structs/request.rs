use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergeRequest {
    pub id: i64,
    pub iid: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub merged_by: Value,
    pub merge_user: Value,
    pub merged_at: Value,
    pub closed_by: Value,
    pub closed_at: Value,
    pub target_branch: String,
    pub source_branch: String,
    pub user_notes_count: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub author: Author,
    pub assignees: Vec<Assignee>,
    pub assignee: Assignee2,
    pub reviewers: Vec<Value>,
    pub source_project_id: i64,
    pub target_project_id: i64,
    pub labels: Vec<Value>,
    pub draft: bool,
    pub work_in_progress: bool,
    pub milestone: Value,
    pub merge_when_pipeline_succeeds: bool,
    pub merge_status: String,
    pub sha: String,
    pub merge_commit_sha: Value,
    pub squash_commit_sha: Value,
    pub discussion_locked: Value,
    pub should_remove_source_branch: Value,
    pub force_remove_source_branch: bool,
    pub reference: String,
    pub references: References,
    pub web_url: String,
    pub time_stats: TimeStats,
    pub squash: bool,
    pub task_completion_status: TaskCompletionStatus,
    pub has_conflicts: bool,
    pub blocking_discussions_resolved: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignee {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignee2 {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct References {
    pub short: String,
    pub relative: String,
    pub full: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeStats {
    pub time_estimate: i64,
    pub total_time_spent: i64,
    pub human_time_estimate: Value,
    pub human_total_time_spent: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskCompletionStatus {
    pub count: i64,
    pub completed_count: i64,
}
