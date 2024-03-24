
use axum::{ extract::State, response:: { Html, IntoResponse } };
use tera::{Context, Tera};
use crate::config::application::Config;
use std::sync::Arc;

pub async fn index(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("index.html", include_str!("../views/index.html")).unwrap();
    let rendered = tera.render("index.html", &Context::new()).unwrap();
    Html(rendered)
}
