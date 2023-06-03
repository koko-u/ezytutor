use chrono::Local;
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewCourse {
    pub tutor_id: Uuid,
    pub course_id: Option<Uuid>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Course {
    pub tutor_id: Uuid,
    pub course_id: Uuid,
    pub course_name: String,
    pub posted_time: NaiveDateTime,
}

impl From<NewCourse> for Course {
    fn from(new_course: NewCourse) -> Self {
        let course_id = new_course.course_id.unwrap_or_else(Uuid::new_v4);
        let posted_time = new_course
            .posted_time
            .unwrap_or_else(|| Local::now().naive_local());
        Self {
            tutor_id: new_course.tutor_id,
            course_id,
            course_name: new_course.course_name,
            posted_time,
        }
    }
}
