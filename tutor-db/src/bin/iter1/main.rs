use std::env;

use dotenv::dotenv;
use env_logger::Env;
use error_stack::IntoReport;
use error_stack::ResultExt;
use errors::AppError;
use log::info;
use models::Course;
use sqlx::PgPool;
use uuid::uuid;

mod errors;
mod models;

#[actix_web::main]
async fn main() -> error_stack::Result<(), AppError> {
    dotenv().into_report().change_context(AppError)?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url = env::var("DATABASE_URL")
        .into_report()
        .change_context(AppError)?;
    let pool = PgPool::connect(&database_url)
        .await
        .into_report()
        .change_context(AppError)?;

    let course_id = uuid!("6465b214-fc07-4ab7-b404-e151a140f4d0");
    let result = sqlx::query_as!(Course,
        r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 WHERE course_id = $1"#,
        course_id
    ).fetch_all(&pool).await.into_report().change_context(AppError)?;

    info!("Courses: {result:#?}");

    Ok(())
}
