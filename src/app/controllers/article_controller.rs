use crate::config::application::Config;
use crate::app::models::article_model::{ Article, ArticleCreate, ArticleEdit };
use crate::db::schema::articles::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;
use serde_json::Value;
use tera::{Context, Tera};
use axum::{ extract::{Path, State}, response:: { Html, IntoResponse, Redirect }, http::{ HeaderMap }, Form};
use barkeel_lib::html::{ get_content_type, response };

pub async fn index(headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    let results = articles
        .load::<Article>(&mut config.database.pool.get().unwrap())
        .expect("Error loading datas");
    if get_content_type(headers) == "application/json" {
        let serialized = serde_json::to_string(&results).unwrap();
        response("application/json",serialized)
    } else {    
        tera.add_raw_template("article/index.html", include_str!("../views/article/index.html")).unwrap();
        let mut context = Context::new();
        context.insert("title", "Article");
        context.insert("base_url", "/articles");
        context.insert("description", "A list of all the users in your account including their name, title, email and role.");
        context.insert("datas", &results);

        let rendered = tera.render("article/index.html", &context).unwrap();
        response("text/html",rendered)
    }
}

pub async fn show(Path(article_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    let result = articles
        .find(article_id)
        .first::<Article>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");
    tera.add_raw_template("article/show.html", include_str!("../views/article/show.html")).unwrap();

    let mut context = Context::new();
    context.insert("data", &result);

    let rendered = tera.render("article/show.html", &context).unwrap();
    Html(rendered)
}

pub async fn new(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("article/new.html", include_str!("../views/article/new.html")).unwrap();

    let mut context = Context::new();
    let config_ref = config.as_ref();
    context.insert("data",&ArticleCreate::new().build_form(config_ref, "/articles"));

    let rendered = tera.render("article/new.html", &context).unwrap();
    Html(rendered)
}

pub async fn create(Form(payload): Form<ArticleEdit>, config: Arc<Config>)  -> Redirect {
    let _inserted_record: Article = diesel::insert_into(articles)
        .values(user_id.eq(payload.user_id))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error inserting data");
    Redirect::to("/articles") 
}

pub async fn edit(Path(article_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("article/edit.html", include_str!("../views/article/edit.html")).unwrap();
    let result = articles
        .find(article_id)
        .first::<Article>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let mut context = Context::new();
    let config_ref = config.as_ref();
    context.insert("data", &result.build_form(config_ref, format!("/articles/{}", article_id).as_str()));

    let rendered = tera.render("article/edit.html", &context).unwrap();
    Html(rendered)
}

pub async fn update( Path(article_id): Path<i32>, Form(payload): Form<ArticleEdit>, config: Arc<Config>) -> Redirect {
    let _updated_record: Article = diesel::update(articles)
        .filter(id.eq(article_id))
        .set(user_id.eq(payload.user_id))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error updating data");
    Redirect::to("/articles") 
}

pub async fn delete(Path(article_id): Path<i32>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(articles)
        .filter(id.eq(article_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/articles") 
}
