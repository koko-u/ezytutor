use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    pub course_id: Uuid,
    pub tutor_id: Uuid,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}
