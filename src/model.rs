use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Card {
    id: i64,
    uid: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    archived: bool,
    title: String,
    asap: bool,
    due_date: Option<DateTime<Utc>>,
    sort_order: f64,
    fifo_order: Option<f64>,
    state: i32,
    condition: i32,
    expires_later: bool,
    parents_count: i32,
    children_count: i32,
    children_done: i32,
    has_blocked_children: bool,
    goals_total: i32,
    goals_done: i32,
    time_spent_sum: i32,
    time_blocked_sum: i32,
    children_number_properties_sum: Option<i32>,
    parent_checklist_ids: Option<Vec<String>>,
    parents_ids: Option<Vec<String>>,
    children_ids: Option<Vec<String>>,
    blocking_card: bool,
    blocked: bool,
    size: Option<i32>,
    size_unit: Option<String>,
    size_text: Option<String>,
    due_date_time_present: bool,
    board_id: i64,
    column_id: i64,
    lane_id: i64,
    owner_id: i64,
    type_id: i64,
    version: i32,
    updater_id: i64,
    completed_on_time: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    last_moved_at: DateTime<Utc>,
    lane_changed_at: DateTime<Utc>,
    column_changed_at: DateTime<Utc>,
    first_moved_to_in_progress_at: DateTime<Utc>,
    last_moved_to_done_at: Option<DateTime<Utc>>,
    sprint_id: Option<i64>,
    external_id: Option<String>,
    comments_total: i32,
    comment_last_added_at: DateTime<Utc>,
    properties: Option<serde_json::Value>,
    counters_recalculated_at: DateTime<Utc>,
    service_id: Option<String>,
    sd_new_comment: bool,
    public: bool,
    share_id: Option<String>,
    share_settings: Option<serde_json::Value>,
    external_user_emails: Option<Vec<String>>,
    calculated_planned_start: Option<DateTime<Utc>>,
    calculated_planned_end: Option<DateTime<Utc>>,
    planned_start: Option<DateTime<Utc>>,
    planned_end: Option<DateTime<Utc>>,
    description_filled: bool,
    estimate_workload: i32,
    tag_ids: Vec<i64>,
    board: Board,
    lane: Lane,
    r#type: Type,
    column: Column,
    tags: Vec<Tag>,
    files: Vec<File>,
    members: Vec<Member>,
    owner: Owner,
    path_data: PathData,
}

#[derive(Debug, Serialize, Deserialize)]
struct Board {
    id: i64,
    title: String,
    external_id: Option<String>,
    card_properties: Option<serde_json::Value>,
    spaces: Vec<Space>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Space {
    id: i64,
    uid: String,
    title: String,
    external_id: Option<String>,
    company_id: i64,
    sort_order: f64,
    path: String,
    parent_entity_uid: Option<String>,
    entity_type: String,
    access: String,
    for_everyone_access_role_id: String,
    board_id: i64,
    space_id: i64,
    top: i64,
    left: i64,
    r#type: i32,
    primary_path: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Lane {
    id: i64,
    title: String,
    sort_order: f64,
    board_id: i64,
    condition: i32,
    external_id: Option<String>,
    default_card_type_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Type {
    id: i64,
    name: String,
    color: i32,
    letter: String,
    company_id: i64,
    archived: bool,
    properties: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Column {
    id: i64,
    uid: String,
    title: String,
    sort_order: f64,
    col_count: i32,
    r#type: i32,
    board_id: i64,
    column_id: Option<String>,
    external_id: Option<String>,
    rules: i32,
    pause_sla: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    id: i64,
    name: String,
    color: i32,
    card_id: i64,
    tag_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    id: i64,
    uid: String,
    name: String,
    title: Option<String>,
    size: i64,
    card_id: i64,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    uploader_id: i64,
    external_id: Option<String>,
    url: String,
    r#type: i32,
    description: Option<String>,
    preview_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Member {
    id: i64,
    uid: String,
    board_id: i64,
    user_id: i64,
    card_id: i64,
    card_subscribed: bool,
    member_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Owner {
    id: i64,
    uid: String,
    user_id: i64,
    board_id: i64,
    card_id: i64,
    card_owner_type: i32,
    owner_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PathData {
    board_id: i64,
    uid: String,
    card_id: i64,
    path: String,
    path_type: i32,
}
