use actix_web::web;

pub fn courses_routes() -> impl FnOnce(&mut web::ServiceConfig) {
    |cfg| {
        cfg.service(
            web::scope("/courses")
                .route("", web::post().to(post_new_course::post_new_course))
                .route(
                    "/{tutor_id}",
                    web::get().to(get_course_for_tutor::get_course_for_tutor),
                )
                .route(
                    "/{tutor_id}/{course_id}",
                    web::get().to(get_course::get_course),
                ),
        );
    }
}

mod get_course;
mod get_course_for_tutor;
mod post_new_course;
