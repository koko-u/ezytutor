use std::env;
use std::net;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use error_stack::IntoReport;
use error_stack::ResultExt;
use log::info;
use tutor_db::errors::AppError;
use tutor_db::scopes::courses::courses_routes;
use tutor_db::states::db_state::DbState;

#[actix_web::main]
async fn main() -> error_stack::Result<(), AppError> {
    dotenv::dotenv().into_report().change_context(AppError)?;
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {addr:?}");

    let database_url = env::var("DATABASE_URL")
        .into_report()
        .change_context(AppError)?;

    let db_state = DbState::new(&database_url).await?;
    let db_state = web::Data::new(db_state);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(db_state.clone())
            .configure(courses_routes())
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
