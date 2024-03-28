
use axum::{  extract::{ State, Path}, response:: { Html, IntoResponse }, http::{header, HeaderMap, StatusCode} };
use tera::{Context, Tera};
use crate::config::application::Config;
use std::sync::Arc;

static THEME_CSS: &str = include_str!("../../public/css/main.css");
static FAVICON: &str = include_str!("../../public/img/favicon.svg");

pub async fn index(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("index.html", include_str!("../views/index.html")).unwrap();
    let rendered = tera.render("index.html", &Context::new()).unwrap();
    Html(rendered)
}

pub async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
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