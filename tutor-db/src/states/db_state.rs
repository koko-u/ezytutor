use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::Course;
use error_stack::IntoReport;
use error_stack::ResultExt;

#[derive(Debug, Clone, derive_more::Deref, derive_more::From)]
pub struct DbState(PgPool);

impl DbState {
    pub async fn new(database_url: &str) -> error_stack::Result<Self, AppError> {
        let pool = PgPool::connect(database_url)
            .await
            .into_report()
            .change_context(AppError)?;

        Ok(Self(pool))
    }

    pub async fn get_course_for_tutor(
        &self,
        tutor_id: Uuid,
    ) -> error_stack::Result<Vec<Course>, AppError> {
        let mut conn = self
            .acquire()
            .await
            .into_report()
            .change_context(AppError)?;

        let courses = sqlx::query_as!(
            Course,
            r#"
                SELECT
                    course_id,
                    tutor_id,
                    course_name,
                    posted_time
                FROM
                    ezy_course_c4
                WHERE
                    tutor_id = $1
            "#,
            tutor_id
        )
        .fetch_all(&mut conn)
        .await
        .into_report()
        .change_context(AppError)?;

        Ok(courses)
    }

    pub async fn get_course(
        &self,
        tutor_id: Uuid,
        course_id: Uuid,
    ) -> error_stack::Result<Option<Course>, AppError> {
        let mut conn = self
            .acquire()
            .await
            .into_report()
            .change_context(AppError)?;

        let course = sqlx::query_as!(
            Course,
            r#"
                SELECT
                    course_id,
                    tutor_id,
                    course_name,
                    posted_time
                FROM
                    ezy_course_c4
                WHERE
                    tutor_id = $1
                    AND
                    course_id = $2
            "#,
            tutor_id,
            course_id
        )
        .fetch_optional(&mut conn)
        .await
        .into_report()
        .change_context(AppError)?;

        Ok(course)
    }

    pub async fn create_course(&self, new_course: Course) -> error_stack::Result<Course, AppError> {
        let mut conn = self
            .acquire()
            .await
            .into_report()
            .change_context(AppError)?;
        let Course {
            course_id,
            tutor_id,
            course_name,
            posted_time,
        } = new_course;

        let created = sqlx::query_as!(
            Course,
            r#"
                INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name, posted_time)
                VALUES ($1, $2, $3, $4)
                RETURNING course_id, tutor_id, course_name, posted_time
            "#,
            course_id,
            tutor_id,
            course_name,
            posted_time
        )
        .fetch_one(&mut conn)
        .await
        .into_report()
        .change_context(AppError)?;

        Ok(created)
    }
}
