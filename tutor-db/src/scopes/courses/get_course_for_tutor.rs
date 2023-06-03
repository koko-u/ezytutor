use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use uuid::Uuid;

use crate::errors::AppResponseError;
use crate::states::db_state::DbState;

pub async fn get_course_for_tutor(
    path: web::Path<Uuid>,
    db_state: web::Data<DbState>,
) -> Result<impl Responder, AppResponseError> {
    let tutor_id = path.into_inner();

    let courses = db_state.get_course_for_tutor(tutor_id).await?;

    Ok(HttpResponse::Ok().json(courses))
}
