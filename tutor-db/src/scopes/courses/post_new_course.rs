use actix_web::http::header;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::errors::AppResponseError;
use crate::models::Course;
use crate::states::db_state::DbState;

pub async fn post_new_course(
    body: web::Json<Course>,
    db_state: web::Data<DbState>,
) -> Result<impl Responder, AppResponseError> {
    let new_course = body.into_inner();

    let created = db_state.create_course(new_course).await?;

    let response = HttpResponse::Created()
        .append_header((
            header::LOCATION,
            format!(
                "/{tutor_id}/{course_id}",
                tutor_id = created.tutor_id,
                course_id = created.course_id
            ),
        ))
        .json(created);

    Ok(response)
}
