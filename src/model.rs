use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: i64,
    pub uid: Option<String>,
    pub created: String,
    pub updated: String,
    pub archived: bool,
    pub title: String,
    pub asap: bool,
    pub due_date: Option<String>,
    pub sort_order: f64,
    pub fifo_order: Option<f64>,
    pub state: i32,
    pub condition: i32,
    pub expires_later: bool,
    pub parents_count: i32,
    pub children_count: i32,
    pub children_done: i32,
    pub has_blocked_children: bool,
    pub goals_total: i32,
    pub goals_done: i32,
    pub time_spent_sum: i32,
    pub time_blocked_sum: i32,
    pub children_number_properties_sum: Option<serde_json::Value>,
    pub parent_checklist_ids: Option<Vec<String>>,
    pub parents_ids: Option<Vec<u64>>,
    pub children_ids: Option<Vec<serde_json::Value>>,
    pub blocking_card: bool,
    pub blocked: bool,
    pub size: Option<serde_json::Value>,
    pub size_unit: Option<String>,
    pub size_text: Option<String>,
    pub due_date_time_present: bool,
    pub board_id: Option<i64>,
    pub column_id: i64,
    pub lane_id: i64,
    pub owner_id: i64,
    pub type_id: i64,
    pub version: i32,
    pub updater_id: i64,
    pub completed_on_time: Option<String>,
    pub completed_at: Option<String>,
    pub last_moved_at: String,
    pub lane_changed_at: String,
    pub column_changed_at: String,
    pub first_moved_to_in_progress_at: Option<String>,
    pub last_moved_to_done_at: Option<String>,
    pub sprint_id: Option<i64>,
    pub external_id: Option<String>,
    pub comments_total: i32,
    pub comment_last_added_at: Option<String>,
    pub properties: Option<serde_json::Value>,
    pub counters_recalculated_at: Option<String>,
    pub service_id: Option<String>,
    pub sd_new_comment: bool,
    pub public: bool,
    pub share_id: Option<String>,
    pub share_settings: Option<serde_json::Value>,
    pub external_user_emails: Option<Vec<String>>,
    pub calculated_planned_start: Option<String>,
    pub calculated_planned_end: Option<String>,
    pub planned_start: Option<String>,
    pub planned_end: Option<String>,
    pub description_filled: bool,
    pub estimate_workload: i32,
    pub tag_ids: Option<Vec<i64>>,
    pub board: Board,
    pub lane: Lane,
    pub r#type: Type,
    pub column: Column,
    pub tags: Option<Vec<Tag>>,
    pub files: Option<Vec<File>>,
    pub members: Option<Vec<Member>>,
    pub owner: Owner,
    pub path_data: PathData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub id: i64,
    pub title: String,
    pub external_id: Option<String>,
    pub card_properties: Option<serde_json::Value>,
    pub spaces: Vec<Space>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Space {
    pub id: i64,
    pub uid: Option<String>,
    pub title: String,
    pub external_id: Option<String>,
    pub company_id: Option<i64>,
    pub sort_order: f64,
    pub path: Option<String>,
    pub parent_entity_uid: Option<String>,
    pub entity_type: String,
    pub access: String,
    pub for_everyone_access_role_id: String,
    pub board_id: Option<i64>,
    pub space_id: i64,
    pub top: i64,
    pub left: i64,
    pub r#type: i32,
    pub primary_path: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lane {
    pub id: i64,
    pub title: String,
    pub sort_order: f64,
    pub board_id: Option<i64>,
    pub condition: i32,
    pub external_id: Option<String>,
    pub default_card_type_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Type {
    pub id: i64,
    pub name: String,
    pub color: i32,
    pub letter: String,
    pub company_id: Option<i64>,
    pub archived: bool,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Column {
    pub id: i64,
    pub uid: Option<String>,
    pub title: String,
    pub sort_order: f64,
    pub col_count: i32,
    pub r#type: i32,
    pub board_id: Option<i64>,
    pub column_id: Option<String>,
    pub external_id: Option<String>,
    pub rules: i32,
    pub pause_sla: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: i32,
    pub card_id: Option<i64>,
    pub tag_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub id: i64,
    pub uid: Option<String>,
    pub name: String,
    pub title: Option<String>,
    pub size: i64,
    pub card_id: Option<i64>,
    pub created: String,
    pub updated: String,
    pub uploader_id: Option<i64>,
    pub external_id: Option<String>,
    pub url: String,
    pub r#type: i32,
    pub description: Option<String>,
    pub preview_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Member {
    pub id: i64,
    pub uid: Option<String>,
    pub board_id: Option<i64>,
    pub user_id: Option<i64>,
    pub card_id: Option<i64>,
    pub card_subscribed: Option<bool>,
    pub member_type: Option<i32>,
    pub full_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Owner {
    pub id: i64,
    pub uid: Option<String>,
    pub user_id: Option<i64>,
    pub board_id: Option<i64>,
    pub card_id: Option<i64>,
    pub card_owner_type: Option<i32>,
    pub owner_type: Option<i32>,
    pub email_blocked: Option<serde_json::Value>,
    pub email_blocked_reason: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathData {
    pub board_id: Option<i64>,
    pub uid: Option<String>,
    pub card_id: Option<i64>,
    pub path: Option<String>,
    pub path_type: Option<i32>,
}
