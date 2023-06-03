use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use uuid::Uuid;

use crate::errors::AppResponseError;
use crate::states::db_state::DbState;

pub async fn get_course(
    path: web::Path<(Uuid, Uuid)>,
    db_state: web::Data<DbState>,
) -> Result<impl Responder, AppResponseError> {
    let (tutor_id, course_id) = path.into_inner();

    let course = db_state.get_course(tutor_id, course_id).await?;

    match course {
        Some(course) => Ok(HttpResponse::Ok().json(course)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
