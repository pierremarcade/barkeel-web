use axum::{ extract::{ State, Path}, routing::{get, post, patch, delete}, Router,  http::{header, HeaderMap, StatusCode}, response::IntoResponse };
use std::sync::Arc;
use tera::{Context, Tera};
use crate::config::application::Config;
use crate::app::controllers::*;

//Add here new route
pub fn routes(config: Arc<Config>) -> Router<Arc<Config>> {
    Router::new()
            .route("/", get(index_controller::index))
            .route("/public/*path", get(handle_assets))
		    .route("/users", get(user_controller::index))
            .route("/users/new", get(user_controller::new))
            .route("/users/:id", get(user_controller::show))
            .route("/users/:id/edit", get(user_controller::edit))
            .route(
                "/articles",
                post({
                    let conflig_clone = config.clone();
                    move |body| article_controller::create(body, conflig_clone)
                }),
            )
            .route(
                "/articles/:id",
                post({
                    let config_clone = config.clone();
                    move |id, body| article_controller::update(id, body, config_clone)
                }),
            )
            .route("/articles", get(article_controller::index))
            .route("/articles/new", get(article_controller::new))
            .route("/articles/:id", get(article_controller::show))
            .route("/articles/:id", delete(article_controller::delete))
            .route("/articles/:id/edit", get(article_controller::edit))
            .fallback(handler_404)
}

async fn handler_404(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    let tera: &Tera = &config.template;
    let rendered = tera.render("404.html", &Context::new()).unwrap();
    (StatusCode::NOT_FOUND, headers, rendered)
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