
use axum::{ extract::State, response:: { Html, IntoResponse } };
use tera::Tera;
use barkeel_lib::app::Config;
use tera::Context;

pub async fn index(State(config): State<Config>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("index.html", include_str!("../views/index.html")).unwrap();
    let rendered = tera.render("index.html", &Context::new()).unwrap();
    Html(rendered)
}