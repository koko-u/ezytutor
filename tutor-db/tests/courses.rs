use std::str::FromStr;

use actix_web::body::MessageBody;
use actix_web::http;
use actix_web::rt::Runtime;
use actix_web::web;
use actix_web::App;
use pretty_assertions::assert_eq;
use sqlx::PgPool;
use tutor_db::models::Course;
use tutor_db::scopes::courses::courses_routes;
use tutor_db::states::db_state::DbState;
use uuid::Uuid;

struct Fixture<'a> {
    runtime: &'a Runtime,
    pool: PgPool,
}

impl<'a> Fixture<'a> {
    fn new(r: &'a Runtime) -> Self {
        dotenv::dotenv().unwrap();
        let database_url = std::env::var("DATABASE_URL").unwrap();

        let pool = r.block_on(PgPool::connect(&database_url)).unwrap();

        Self { runtime: r, pool }
    }
}
impl<'a> Drop for Fixture<'a> {
    fn drop(&mut self) {
        let mut conn = self.runtime.block_on(self.pool.acquire()).unwrap();
        self.runtime
            .block_on(sqlx::query!("TRUNCATE TABLE ezy_course_c4").execute(&mut conn))
            .unwrap();
    }
}

#[test]
fn post_course() {
    let r = Runtime::new().unwrap();
    let fixture = Fixture::new(&r);

    let db_state = DbState::from(fixture.pool.clone());
    let db_state = web::Data::new(db_state);

    let app = r.block_on(actix_web::test::init_service(
        App::new().app_data(db_state).configure(courses_routes()),
    ));

    let new_course = Course {
        tutor_id: Uuid::from_str("512e0b53-d9d3-4ed7-b758-1f65b1c14a4f").unwrap(),
        course_id: Uuid::from_str("dcfe2fd4-c12d-4218-90fe-6a18d65a035f").unwrap(),
        course_name: "Course 1".to_string(),
        posted_time: None,
    };
    let req = actix_web::test::TestRequest::post()
        .uri("/courses")
        .set_json(new_course.clone())
        .to_request();
    let res = r.block_on(actix_web::test::call_service(&app, req));

    assert_eq!(res.status(), http::StatusCode::CREATED);

    let body = res.into_body();
    let bytes = body.try_into_bytes().unwrap();
    let s = String::from_utf8(bytes.to_vec()).unwrap();

    let created: Course = serde_json::from_str(&s).unwrap();

    assert_eq!(created, new_course);
}
