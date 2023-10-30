use crate::data_types::structs::ErrorMessage;
use actix_web::Result;
use sqlx::{Error, PgPool};
use std::env;

pub async fn connect() -> Result<PgPool, ErrorMessage> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool_result = PgPool::connect(&database_url).await;

    match pool_result {
        Ok(db) => {
            Ok(db)
        }
        Err(e) => {
            let err_msg = match e {
                Error::Configuration(e) => format!("Configuration error: {}", e),
                Error::Database(e) => format!("Database error: {}", e),
                _ => format!("Unknown Error: {}", e),
            };
            Err(ErrorMessage::new(&err_msg))
        }
    }
}
