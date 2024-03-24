use axum::{ extract::Path, routing::get, Router,  http::{header, HeaderMap, StatusCode}, response::IntoResponse };
use std::sync::Arc;
use crate::config::application::Config;
use crate::app::controllers::*;

//Add here new route
pub fn routes() -> Router<Arc<Config>> {
    Router::new()
            .route("/", get(index_controller::index))
            .route("/public/*path", get(handle_assets))
		    .route("/users", get(user_controller::index))
            .route("/users/:id", get(user_controller::show))
            .route("/users/:id/edit", get(user_controller::edit))
            .route("/users/new", get(user_controller::new))
            .fallback(handler_404)
}


async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

static THEME_CSS: &str = include_str!("../public/css/main.css");
static FAVICON: &str = include_str!("../public/img/favicon.svg");


async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "css/main.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, THEME_CSS)
    } else if path == "img/favicon.svg" {
        (StatusCode::OK, headers, FAVICON)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}