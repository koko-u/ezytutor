use std::net;

use actix_web::middleware;
use actix_web::App;
use actix_web::HttpServer;
use error_stack::IntoReport;
use error_stack::ResultExt;
use log::info;
use tutor_nodb::errors::AppError;

#[actix_web::main]
async fn main() -> error_stack::Result<(), AppError> {
    dotenv::dotenv().into_report().change_context(AppError)?;
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {addr:?}");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
        //.configure(health_routes)
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
