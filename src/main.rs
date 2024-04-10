pub mod models;
pub mod views;

use std::fs::read_to_string;

use models::{EditTodo, NewTodo, Todo};
use views::Hypermedia;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Form, Json,
};
use lfml::Render;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    let app = axum::Router::new()
        .route(
            "/static/*f",
            axum::routing::get(|Path(f): Path<String>| async move {
                let Ok(data) = read_to_string(format!("./static/{}", &f)) else {
                    return StatusCode::NOT_FOUND.into_response();
                };
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        mime_guess::from_path(&f).first_or_octet_stream().as_ref(),
                    )],
                    data,
                )
                    .into_response()
            }),
        )
        .route(
            "/",
            axum::routing::get(
                |headers: HeaderMap, State(pool): State<PgPool>| async move {
                    let Ok(todos) = Todo::get_all(&pool).await else {
                        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };
                    hypermedia_request(headers, views::index(&todos)).into_response()
                },
            ),
        )
        .route(
            "/scratch",
            axum::routing::get(
                || async move {
                    Hypermedia::Document(views::scratch()).markup().into_response()
                },
            ),
        )
        .route(
            "/todos",
            axum::routing::get(|State(pool): State<PgPool>| async move {
                let Ok(todos) = Todo::get_all(&pool).await else {
                    return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
                };
                Json(todos).into_response()
            }),
        )
        .route(
            "/todos/new",
            axum::routing::post(
                |State(pool): State<PgPool>, Form(NewTodo { name }): Form<NewTodo>| async move {
                    let Ok(todo) = Todo::new(&pool, name).await else {
                        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };
                    todo.render_display().into_response()
                },
            ),
        )
        .route(
            "/todos/:ident",
            axum::routing::get(
                |Path(ident): Path<uuid::Uuid>, State(pool): State<PgPool>| async move {
                    let Ok(todo) = Todo::get(&pool, ident).await else {
                        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };
                    todo.render_display().into_response()
                }
            ).put(
                |State(pool): State<PgPool>, Form(EditTodo { name, ident, completed }): Form<EditTodo>| async move {
                    let Ok(todo) = Todo::edit(&pool, ident, name, completed).await else {
                        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };
                    todo.render_display().into_response()
                }
            ).delete(
                |Path(ident): Path<uuid::Uuid>, State(pool): State<PgPool>| async move {
                    let Ok(_) = Todo::delete(&pool, ident).await else {
                        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };
                    ().into_response()
                }
            )
        )
        .route(
            "/todos/:ident/edit",
            axum::routing::get(
                |Path(ident): Path<uuid::Uuid>, State(pool): State<PgPool>| async move {
                    let Ok(todo) = Todo::get(&pool, ident).await else {
                        return StatusCode::PRECONDITION_REQUIRED.into_response();
                    };
                    todo.render_edit().into_response()
                },
            ),
        )
        .with_state(pool);

    #[cfg(debug_assertions)]
    let app = app.layer(
        tower_livereload::LiveReloadLayer::new().request_predicate::<axum::body::Body, _>(
            |req: &axum::http::Request<_>| !req.headers().contains_key("HX-Request"),
        ),
    );

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(),
        app,
    )
    .await?;
    Ok(())
}

fn hypermedia_request<T: lfml::Render>(headers: HeaderMap, templ: T) -> lfml::Markup {
    {
        if !headers.contains_key("HX-Request") {
            Hypermedia::Document(templ)
        } else {
            Hypermedia::Fragment(templ)
        }
    }
    .markup()
}
