use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: Value,
    pub body: String,
    pub attachment: Value,
    pub author: Author,
    pub created_at: String,
    pub updated_at: String,
    pub system: bool,
    pub noteable_id: i64,
    pub noteable_type: String,
    pub commit_id: Option<Value>,
    pub position: Option<Position>,
    pub resolvable: bool,
    pub resolved: Option<bool>,
    pub resolved_by: Option<Value>,
    pub resolved_at: Option<Value>,
    pub confidential: bool,
    pub noteable_iid: i64,
    pub commands_changes: CommandsChanges,
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
pub struct Position {
    pub base_sha: String,
    pub start_sha: String,
    pub head_sha: String,
    pub old_path: String,
    pub new_path: String,
    pub position_type: String,
    pub old_line: Value,
    pub new_line: i64,
    pub line_range: LineRange,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineRange {
    pub start: Start,
    pub end: End,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Start {
    pub line_code: String,
    #[serde(rename = "type")]
    pub type_field: Value,
    pub old_line: Value,
    pub new_line: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct End {
    pub line_code: String,
    #[serde(rename = "type")]
    pub type_field: Value,
    pub old_line: Value,
    pub new_line: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandsChanges {}
