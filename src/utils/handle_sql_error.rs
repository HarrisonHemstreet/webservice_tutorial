use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::HttpResponse;
use sqlx::Error;

pub fn handle_sql_error(e: Error) -> HttpResponse {
    match e {
        Error::Database(err) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("Database error: {}", err)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::RowNotFound => HttpResponse::NotFound()
            .status(StatusCode::NOT_FOUND)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("Row not found")))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::Decode(err) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("Decode error: {}", err)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::Io(err) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("IO error: {}", err)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::Migrate(err) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("Migrate error: {}", err)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::PoolTimedOut => HttpResponse::GatewayTimeout()
            .status(StatusCode::GATEWAY_TIMEOUT)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("error: Connection pool timed out")))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::PoolClosed => HttpResponse::ServiceUnavailable()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("error: Connection pool is closed")))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::WorkerCrashed => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("error: Worker has crashed")))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::Tls(err) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("TLS error: {}", err)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::Protocol(err) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("Protocol error: {}", err)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::ColumnNotFound(column) => HttpResponse::BadRequest()
            .status(StatusCode::BAD_REQUEST)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("Column not found: {}", column)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::ColumnDecode { index, source } => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!(
                    "Error decoding column at index {}: {}",
                    index, source
                )))
                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::Configuration(err) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("Configuration error: {}", err)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        Error::TypeNotFound { type_name } => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!("Type not found: {}", type_name)))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),

        _ => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(format!(
                    "error: Non-exhaustive error variant encountered"
                )))
                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),
    }
}
