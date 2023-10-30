use crate::data_types::structs::{Blog, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, post, put, web::Query, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/blog")]
async fn create_blog(blog: Json<Blog>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                r#"
                    WITH new_row AS (
                        INSERT INTO blog
                            (
                                title,
                                slug,
                                content,
                                image_link,
                                thumbnail_link,
                                featured
                            )
                        VALUES (
                            $1,
                            $2,
                            $3,
                            $4,
                            $5,
                            $6
                        )
                        RETURNING
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY HH12:MI AM'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY HH12:MI AM'))
                            ) as edited
                    )
                    SELECT
                        new_row.id,
                        new_row.title,
                        new_row.slug,
                        new_row.content,
                        new_row.image_link,
                        new_row.thumbnail_link,
                        new_row.featured,
                        new_row.created,
                        new_row.edited
                    FROM new_row
                "#,
                blog.title,
                blog.slug,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[get("/blog/featured")]
async fn get_featured_blogs() -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<Blog>, Error> = sqlx::query_as!(
                Blog,
                r#"
                    SELECT
                        id,
                        title,
                        slug,
                        content,
                        image_link,
                        thumbnail_link,
                        featured,
                        (
	                        trim(to_char(created, 'DD')) || ' ' ||
	                        trim(to_char(created, 'Month')) || ' ' ||
	                        trim(to_char(created, 'YYYY HH12:MI AM'))
                        ) as created,
                        (
	                        trim(to_char(edited, 'DD')) || ' ' ||
	                        trim(to_char(edited, 'Month')) || ' ' ||
	                        trim(to_char(edited, 'YYYY HH12:MI AM'))
                        ) as edited
                    FROM blog
                    WHERE featured = TRUE
                    LIMIT 2;
                "#
            )
            .fetch_all(&pg)
            .await;

            match returned {
                Ok(record) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[get("/blog")]
async fn get_blog_by_id_or_all(Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Blog, Error> = sqlx::query_as!(
                    Blog,
                    r#"
                        SELECT
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY HH12:MI AM'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY HH12:MI AM'))
                            ) as edited
                        FROM blog
                        WHERE id = $1
                        LIMIT 1;
                    "#,
                    id.id
                )
                .fetch_one(&pg)
                .await;

                match returned {
                    Ok(record) => HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json")
                        .body(
                            serde_json::to_string(&Json(record))
                                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                        ),

                    Err(e) => handle_sql_error(e),
                }
            }
            Err(e) => HttpResponse::InternalServerError()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .body(e.message),
        }
    } else {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Vec<Blog>, Error> = sqlx::query_as!(
                    Blog,
                    r#"
                        SELECT
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY HH12:MI AM'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY HH12:MI AM'))
                            ) as edited
                        FROM blog;
                    "#,
                )
                .fetch_all(&pg)
                .await;

                match returned {
                    Ok(record) => HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json")
                        .body(
                            serde_json::to_string(&Json(record))
                                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                        ),

                    Err(e) => handle_sql_error(e),
                }
            }
            Err(e) => HttpResponse::InternalServerError()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .body(e.message),
        }
    }
}

#[put("/blog")]
async fn update_blog(blog: Json<Blog>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                r#"
                    WITH new_row AS (
                        INSERT INTO blog (id, title, slug, content, image_link, thumbnail_link, featured)
                        VALUES (
                            $1, $2, $3, $4, $5, $6, $7
                        )
                        ON CONFLICT (id)
                        DO UPDATE SET
                            id = EXCLUDED.id,
                            title = EXCLUDED.title,
                            slug = EXCLUDED.slug,
                            content = EXCLUDED.content,
                            image_link = EXCLUDED.image_link,
                            thumbnail_link = EXCLUDED.thumbnail_link,
                            featured = EXCLUDED.featured,
                            edited = NOW()
                        RETURNING
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY HH12:MI AM'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY HH12:MI AM'))
                            ) as edited
                    )
                    SELECT
                        new_row.id,
                        new_row.title,
                        new_row.slug,
                        new_row.content,
                        new_row.image_link,
                        new_row.thumbnail_link,
                        new_row.featured,
                        new_row.created,
                        new_row.edited
                    FROM new_row
                "#,
                blog.id,
                blog.title,
                blog.slug,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured,
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[delete("/blog")]
async fn delete_blog(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> =
                sqlx::query_as!(Blog, "DELETE FROM blog WHERE id = $1;", id.id)
                    .execute(&pg)
                    .await;

            match returned {
                Ok(_) => HttpResponse::NoContent()
                    .status(StatusCode::NO_CONTENT)
                    .content_type("application/json")
                    .finish(),

                Err(e) => handle_sql_error(e),
            }
        }

        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}
