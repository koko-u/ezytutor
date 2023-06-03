use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::states::health::HealthState;

pub fn health_routes(health_state: web::Data<HealthState>) -> impl FnOnce(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(
            web::scope("/health")
                .app_data(health_state)
                .route("", web::get().to(health_check)),
        );
    }
}

async fn health_check(app_state: web::Data<HealthState>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;

    HttpResponse::Ok().json(response)
}
