use crate::config::application::Config;
use crate::app::models::user_model::{ User, UserCreate };
use crate::db::schema::users::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;
use serde_json::Value;
use tera::{Context, Tera};
use axum::{ extract::{Json, Path, State}, response:: { Html, IntoResponse, Redirect }, http::{ HeaderMap } };
use barkeel_lib::html::{ get_content_type, response };

pub async fn index(headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    let results = users
        .load::<User>(&mut config.database.pool.get().unwrap())
        .expect("Error loading datas");
    if get_content_type(headers) == "application/json" {
        let serialized = serde_json::to_string(&results).unwrap();
        response("application/json",serialized)
    } else {    
        tera.add_raw_template("user/index.html", include_str!("../views/user/index.html")).unwrap();
        let mut context = Context::new();
        context.insert("title", "User");
        context.insert("description", "A list of all the users in your account including their name, title, email and role.");
        context.insert("datas", &results);

        let rendered = tera.render("user/index.html", &context).unwrap();
        response("text/html",rendered)
    }
}

pub async fn show(Path(user_id): Path<i64>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    let result = users
        .find(user_id)
        .first::<User>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");
    tera.add_raw_template("user/show.html", include_str!("../views/user/show.html")).unwrap();

    let mut context = Context::new();
    context.insert("data", &result);

    let rendered = tera.render("user/show.html", &context).unwrap();
    Html(rendered)
}

pub async fn new(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("user/new.html", include_str!("../views/user/new.html")).unwrap();

    let mut context = Context::new();
    let config_ref = config.as_ref();
    context.insert("data",&UserCreate::new().build_form(config_ref, "user/index"));

    let rendered = tera.render("user/new.html", &context).unwrap();
    Html(rendered)
}

pub async fn create(Json(payload): Json<Value>, State(config): State<Arc<Config>>)  -> Redirect {
    let data: User = serde_json::from_value(payload).unwrap();
    let _inserted_record: User = diesel::insert_into(users)
        .values((id.eq(data.id), name.eq(data.name)))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error inserting data");
    Redirect::to("user/index") 
}

pub async fn edit(Path(user_id): Path<i64>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("user/edit.html", include_str!("../views/user/edit.html")).unwrap();
    let result = users
        .find(user_id)
        .first::<User>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let mut context = Context::new();
    let config_ref = config.as_ref();
    context.insert("data", &result.build_form(config_ref, "user/index"));

    let rendered = tera.render("user/edit.html", &context).unwrap();
    Html(rendered)
}

pub async fn update( Path(user_id): Path<i64>, Json(payload): Json<Value>, State(config): State<Arc<Config>>) -> Redirect {
    let data: User = serde_json::from_value(payload).unwrap();
    let _updated_record: User = diesel::update(users)
        .filter(id.eq(user_id))
        .set((id.eq(data.id), name.eq(data.name)))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error updating data");
    Redirect::to("user/index") 
}

pub async fn delete(Path(user_id): Path<i64>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(users)
        .filter(id.eq(user_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("user/index") 
}
