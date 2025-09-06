use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct Course {
    pub id: i64,
    pub name: String,
    pub rss_url: String,
    pub color: i32,
    pub role_id: String,
    pub last_announcement: Option<i64>,
}
