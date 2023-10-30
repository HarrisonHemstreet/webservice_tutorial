use actix_web::{web, App, HttpServer};
use std::io::Result;

pub mod data_types;
pub mod db;
pub mod routes;
pub mod utils;
pub mod middleware;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(move || {

        App::new()
            .service(routes::blog())
            .wrap(middleware::handle_cors())
            .service(routes::auth())
            .service(
                web::scope("/api/v1")
                    .wrap(middleware::JWTAuth)
                    .wrap(middleware::CaptureUri)
                    .service(routes::auth())
                    .service(routes::blog())
                    .service(routes::tag())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
