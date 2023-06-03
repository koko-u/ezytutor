use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use uuid::Uuid;

use crate::model::courses::Course;
use crate::model::courses::NewCourse;
use crate::states::courses::CoursesSate;

pub fn courses_routes(
    courses_state: web::Data<CoursesSate>,
) -> impl FnOnce(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(
            web::scope("/courses")
                .app_data(courses_state)
                .route("/{tutor_id}", web::get().to(get_courses_of_tutor))
                .route("/{tutor_id}/{course_id}", web::get().to(get_course))
                .route("", web::post().to(new_course)),
        );
    }
}

async fn get_courses_of_tutor(
    path: web::Path<Uuid>,
    courses_state: web::Data<CoursesSate>,
) -> impl Responder {
    let tutor_id = path.into_inner();
    let courses = courses_state.lock().unwrap();
    let courses = courses
        .iter()
        .filter(|course| course.tutor_id == tutor_id)
        .cloned()
        .collect::<Vec<_>>();
    HttpResponse::Ok().json(courses)
}

async fn get_course(
    path: web::Path<(Uuid, Uuid)>,
    courses_state: web::Data<CoursesSate>,
) -> impl Responder {
    let (tutor_id, course_id) = path.into_inner();
    let courses = courses_state.lock().unwrap();
    let course = courses
        .iter()
        .find(|course| course.tutor_id == tutor_id && course.course_id == course_id)
        .cloned();

    match course {
        Some(course) => HttpResponse::Ok().json(course),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn new_course(
    new_course: web::Json<NewCourse>,
    courses_state: web::Data<CoursesSate>,
) -> impl Responder {
    let course: Course = new_course.into_inner().into();
    let mut courses = courses_state.lock().unwrap();
    courses.push(course);

    HttpResponse::Created().finish()
}
