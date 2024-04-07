use crate::config::application::Config;
use crate::app::models::user::{ User, UserForm, UserFormEdit };
use crate::db::schema::users::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;
use tera::{Context, Tera};
use axum::{ extract::{Path, State, Query}, response:: { IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use crate::app::utils::{ get_content_type, csrf_token_is_valid, response::Response, pagination::PaginationQuery };
use crate::app::controllers::error_controller;

pub async fn index(Query(pagination): Query<PaginationQuery>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    let current_page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (current_page - 1) * per_page;
    let total_count: i64 = get_total_count(config.clone());
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    match users.limit(per_page as i64).offset(offset as i64).load::<User>(&mut config.database.pool.get().unwrap()) {
        Ok(results) => {
            if get_content_type(headers) == "application/json" {
                let serialized = serde_json::to_string(&results).unwrap();
                Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized}
            } else {    
                let template_path = "user/index.html";
                let template_content = include_str!("../views/user/index.html");
                let _ = tera.add_raw_template(template_path, template_content);
                let mut context = Context::new();
                context.insert("title", "User");
                context.insert("base_url", "/users");
                context.insert("description", "A list of all the users.");
                context.insert("datas", &results);
                context.insert("total_pages", &total_pages);
                context.insert("current_page", &current_page);
                context.insert("offset", &offset);
                context.insert("per_page", &per_page);
        
                let rendered = tera.render("user/index.html", &context).unwrap();
                Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
            }
        },
        Err(err) => {
            error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string())
        }
    }
}


fn get_total_count(config: Arc<Config>) -> i64 {
    match users.count().get_result(&mut config.database.pool.get().unwrap()) {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Error counting users: {}", e);
            0 
        }
    }
}

pub async fn show(Path(param_id): Path<i64>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    match users.find(param_id).first::<User>(&mut config.database.pool.get().unwrap()) {
        Ok(result) => {
            tera.add_raw_template("user/show.html", include_str!("../views/user/show.html")).unwrap();
            let mut context = Context::new();
            context.insert("data", &result);
            context.insert("title", "User");
            context.insert("description", "User's Detail");
            let rendered = tera.render("user/show.html", &context).unwrap();
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
        },
        _ => {
            error_controller::render_404(config)
        }
    }
}

pub async fn new(headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("user/new.html", include_str!("../views/user/new.html")).unwrap();

    let mut context = Context::new();
    let config_ref = config.as_ref();
    context.insert("data",&UserForm::new().build_form(config_ref, headers, "/users"));

    let rendered = tera.render("user/new.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn create(headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<UserFormEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _inserted_record: User = diesel::insert_into(users)
            .values(name.eq(payload.name))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error inserting data");
    }
    Redirect::to("/users") 
}

pub async fn edit(headers: HeaderMap, Path(param_id): Path<i64>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("user/edit.html", include_str!("../views/user/edit.html")).unwrap();
    let result = users
        .find(param_id)
        .first::<User>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let mut context = Context::new();
    let config_ref = config.as_ref();
    context.insert("data", &result.build_form(config_ref, headers, format!("/users/{}", param_id).as_str()));

    let rendered = tera.render("user/edit.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn update(headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i64>, Form(payload): Form<UserFormEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _updated_record: User = diesel::update(users)
            .filter(id.eq(param_id))
            .set(name.eq(payload.name))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error updating data");
    }
    Redirect::to("/users") 
}

pub async fn delete(Path(param_id): Path<i64>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(users)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/users") 
}
