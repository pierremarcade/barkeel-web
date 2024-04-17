use crate::config::application::Config;
use crate::app::models::customer::{ Customer, CustomerCreate, CustomerEdit };
use crate::db::schema::customers::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;
use tera::{Context, Tera};
use axum::{ extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use crate::app::utils::{ get_content_type, csrf_token_is_valid, response::Response, pagination::{ PaginationQuery, Pagination } };
use crate::app::controllers::error_controller;

pub async fn index(Query(pagination_query): Query<PaginationQuery>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let total_results: i64 = get_total(config.clone());
    let pagination = Pagination::new(pagination_query, total_results);
    match customers.limit(pagination.per_page as i64).offset(pagination.offset as i64).load::<Customer>(&mut config.database.pool.get().unwrap()) {
        Ok(results) => {
            if get_content_type(headers) == "application/json" {
                render_json(config, results)
            } else {    
                render_html(config, results, pagination)
            }
        },
        Err(err) => {
            error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string())
        }
    }
}

fn render_html(config: Arc<Config>, results: Vec<Customer>, pagination: Pagination) -> Response<'static> {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    let template_path = "customer/index.html";
    let template_content = include_str!("../views/customer/index.html");
    let result = tera.add_raw_template(template_path, template_content);
    match result {
        Ok(_) => {},
        Err(err) => {
            return error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string());
        }
    }
    let mut context = Context::new();
    context.insert("title", "Customer");
    context.insert("base_url", "/customers");
    context.insert("description", "A list of all the customers.");
    context.insert("datas", &results);
    context.insert("total_pages", &pagination.total_pages);
    context.insert("current_page", &pagination.current_page);
    context.insert("current_page_string", &pagination.current_page.to_string());
    context.insert("offset", &pagination.offset);
    context.insert("per_page", &pagination.per_page);
    context.insert("page_numbers", &pagination.generate_page_numbers());

    let rendered = tera.render("customer/index.html", &context);
    match rendered {
        Ok(result) => {
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: result}
        },
        Err(err) => {
            error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string())
        }
    }
}

fn render_json(config: Arc<Config>, results: Vec<Customer>) -> Response<'static> {
    let rendered =  match  serde_json::to_string(&results) {
        Ok(serialized) => {
            return Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized};
        },
        Err(err) => {
            return error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string());
        }
    };
    rendered
}

fn get_total(config: Arc<Config>) -> i64 {
    match customers.count().get_result(&mut config.database.pool.get().unwrap()) {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Error counting customers: {}", e);
            0 
        }
    }
}

fn get_total_count(config: Arc<Config>) -> i64 {
    match customers.count().get_result(&mut config.database.pool.get().unwrap()) {
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
    match customers.find(param_id).first::<Customer>(&mut config.database.pool.get().unwrap()) {
        Ok(result) => {
            tera.add_raw_template("customer/show.html", include_str!("../views/customer/show.html")).unwrap();
            let mut context = Context::new();
            context.insert("data", &result);
            context.insert("title", "Customer");
            context.insert("description", "Customer's Detail");
            let rendered = tera.render("customer/show.html", &context).unwrap();
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
    tera.add_raw_template("customer/new.html", include_str!("../views/customer/new.html")).unwrap();

    let mut context = Context::new();
    let config_ref = config.as_ref();
    context.insert("data",&CustomerCreate::new().build_form(config_ref, headers, "/customers"));

    let rendered = tera.render("customer/new.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn create(headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<CustomerEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _inserted_record: Customer = diesel::insert_into(customers)
            .values((customer_id.eq(payload.customer_id), name.eq(payload.name), agency_id.eq(payload.agency_id), sage_id.eq(payload.sage_id), agency_name.eq(payload.agency_name), surikate_option.eq(payload.surikate_option), modified_at.eq(payload.modified_at), team_id.eq(payload.team_id), primary_contact_id.eq(payload.primary_contact_id), supervision_or_backup_contract.eq(payload.supervision_or_backup_contract), host_group_id.eq(payload.host_group_id)))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error inserting data");
    }
    Redirect::to("/customers") 
}

pub async fn edit(headers: HeaderMap, Path(param_id): Path<i64>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("customer/edit.html", include_str!("../views/customer/edit.html")).unwrap();
    let result = customers
        .find(param_id)
        .first::<Customer>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let mut context = Context::new();
    let config_ref = config.as_ref();
    context.insert("data", &result.build_form(config_ref, headers, format!("/customers/{}", param_id).as_str()));

    let rendered = tera.render("customer/edit.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn update(headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i64>, Form(payload): Form<CustomerEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _updated_record: Customer = diesel::update(customers)
            .filter(id.eq(param_id))
            .set((customer_id.eq(payload.customer_id), name.eq(payload.name), agency_id.eq(payload.agency_id), sage_id.eq(payload.sage_id), agency_name.eq(payload.agency_name), surikate_option.eq(payload.surikate_option), modified_at.eq(payload.modified_at), team_id.eq(payload.team_id), primary_contact_id.eq(payload.primary_contact_id), supervision_or_backup_contract.eq(payload.supervision_or_backup_contract), host_group_id.eq(payload.host_group_id)))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error updating data");
    }
    Redirect::to("/customers") 
}

pub async fn delete(Path(param_id): Path<i64>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(customers)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/customers") 
}
