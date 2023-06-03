use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub course_id: Uuid,
    pub tutor_id: Uuid,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}
