use std::net;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use error_stack::IntoReport;
use error_stack::ResultExt;
use log::info;
use tutor_nodb::errors::AppError;
use tutor_nodb::scopes::courses::courses_routes;
use tutor_nodb::scopes::health::health_routes;
use tutor_nodb::states::courses::CoursesSate;
use tutor_nodb::states::health::HealthState;

#[actix_web::main]
async fn main() -> error_stack::Result<(), AppError> {
    dotenv::dotenv().into_report().change_context(AppError)?;
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {addr:?}");

    let health_state = web::Data::new(HealthState::new("I'm good. You've already asked me"));
    let courses_state = web::Data::new(CoursesSate::default());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .configure(health_routes(health_state.clone()))
            .configure(courses_routes(courses_state.clone()))
    })
    .bind(addr)
    .into_report()
    .change_context(AppError)?
    .run()
    .await
    .into_report()
    .change_context(AppError)?;

    Ok(())
}
