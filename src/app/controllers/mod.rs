use axum::http::{ HeaderMap, header };
use barkeel_lib::session::CSRFManager;
use cookie::Cookie;
use tera::Context;
use crate::config::application::Config;
use crate::config::constants::{DEFAULT_LOCALE, LOCALE_COOKIE_NAME, SESSION_COOKIE_NAME};
use unic_langid::LanguageIdentifier;
use barkeel_lib::app::pagination::RequestQuery;

pub trait CrudViewTrait {
    fn index_view(tera: &mut tera::Tera) -> String {
        let _ = tera.add_raw_template("crud_index", include_str!("../views/crud/index.html"));
        "crud_index".to_string()
    }

    fn show_view(tera: &mut tera::Tera) -> String {
        let _ = tera.add_raw_template("crud_show", include_str!("../views/crud/show.html"));
        "crud_show".to_string()
    }

    fn form_view(tera: &mut tera::Tera) -> String {
        let _ = tera.add_raw_template("crud_form", include_str!("../views/crud/form.html"));
        "crud_form".to_string()
    }
}

pub fn get_content_type(headers: HeaderMap) -> String {
    let header_value = headers.get("Content-Type");
    let mut content_type = String::new();
    if let Some(header_value) = header_value {
        content_type = header_value.to_str().unwrap().to_string();
    }
    content_type
}

pub fn get_locale(headers: HeaderMap, request_query: Option<RequestQuery>) -> LanguageIdentifier {
    if let Some(query) = request_query {
        if let Some(locale) = query.locale {
            return locale.parse().expect("Parsing failed.");
        }
    }
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in Cookie::split_parse(cookie_str) {
                let cookie = cookie.unwrap();
                if cookie.name() == LOCALE_COOKIE_NAME {
                    return cookie.value().to_string().parse().expect("Parsing failed.");
                }
            }
        }
    }
    DEFAULT_LOCALE.to_string().parse().expect("Parsing failed.")
}

pub fn is_csrf_token_valid(headers: HeaderMap, config: Config, csrf_token: String) -> bool {
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in Cookie::split_parse(cookie_str) {
                let cookie = cookie.unwrap();
                if cookie.name() == SESSION_COOKIE_NAME {
                    let csrf_manager: &CSRFManager = &config.csrf_manager;
                    if csrf_manager.is_csrf_token_valid(cookie.value().to_string(), csrf_token.clone()) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[macro_export]
macro_rules! crud {
    ($resource:ident, $view:ident) => {
        index!($resource, $view);
        new!($resource, $view);
        edit!($resource, $view);
        show!($resource, $view);
        delete!($resource);
        create!($resource, $view); 
        update!($resource, $view);
    };
}

#[macro_export]
macro_rules! create {
    ($resource:ident, $view:ident) => {
        pub async fn create(Extension(mut current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Config>, Form(payload): Form<CrudForm>) -> impl IntoResponse {
            let locale = get_locale(headers.clone(), None);
            if is_csrf_token_valid(headers.clone(), config.clone(), payload.clone().csrf_token) {
                let table_name = stringify!($resource);
                let link_name = table_name.to_kebab_case();
                match payload.validate() {
                    Ok(_) => {
                        if let Some(user) = current_user.get_user().await {
                            let _inserted_record: CrudModel = diesel::insert_into($resource)
                            .values(insert_values(payload, user.clone()))
                            .get_result(&mut config.database.pool.get().unwrap())
                            .expect(&LOCALES.lookup(&locale, "error_insert").to_string());
                        }
                        Redirect::to(format!("/{}", link_name).as_str()).into_response()
                    },
                    Err(e) => {
                        let form = payload.build_form(&config.clone(), headers, format!("/{}", link_name).as_str());
                        render_form!(form, $view, config, current_user, Some(e.clone()))
                    }
                }
            } else {
                let serialized = serde_json::to_string(&LOCALES.lookup(&locale, "invalid_csrf_token").to_string()).unwrap();
                render_json!(StatusCode::BAD_REQUEST, serialized) 
            }
        }
    }
}

#[macro_export]
macro_rules! update {
    ($resource:ident, $view:ident) => {
        pub async fn update(Extension(mut current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Config>, Path(param_id): Path<i32>, Form(payload): Form<CrudForm>) -> impl IntoResponse {
            let locale = get_locale(headers.clone(), None);
            if is_csrf_token_valid(headers.clone(), config.clone(), payload.clone().csrf_token) {
                let table_name = stringify!($resource);
                let link_name = table_name.to_kebab_case();
                match payload.validate() {
                    Ok(_) => {
                        if let Some(user) = current_user.get_user().await {
                            let _updated_record: CrudModel = diesel::update($resource)
                                .filter(id.eq(param_id))
                                .set(update_values(payload, user.clone()))
                                .get_result(&mut config.database.pool.get().unwrap())
                                .expect(&LOCALES.lookup(&locale, "error_update").to_string());
                        }
                        Redirect::to(format!("/{}", link_name).as_str()).into_response()
                    },
                    Err(e) => {
                        let form = payload.build_form(&config.clone(), headers, format!("/{}", link_name).as_str());
                        render_form!(form, $view, config, current_user, Some(e.clone()))
                    }
                }
            } else {
                let serialized = serde_json::to_string(&LOCALES.lookup(&locale, "invalid_csrf_token").to_string()).unwrap();
                render_json!(StatusCode::BAD_REQUEST, serialized) 
            }
        }
    }
}

#[macro_export]
macro_rules! index {
    ($resource:ident, $view:ident) => {
        pub async fn index(State(config): State<Config>, Extension(current_user): Extension<AuthState>, Query(request_query): Query<RequestQuery>, headers: HeaderMap) -> impl IntoResponse {
            let locale = get_locale(headers.clone(), Some(request_query.clone()));
            let total_results: i64 = get_total!(config, $resource);
            let pagination = Pagination::new(request_query, total_results);
            match $resource.limit(pagination.per_page as i64).offset(pagination.offset as i64).load::< CrudModel >(&mut config.database.pool.get().unwrap()) {
                Ok(results) => {
                    if get_content_type(headers) == "application/json" {
                        render_json!(StatusCode::OK, results)
                    } else {
                        let table_name = stringify!($resource);
                        let link_name = table_name.to_kebab_case();
                        let model_class = table_name.to_class_case();
                        let mut context = prepare_tera_context(current_user).await;
                        let args = {
                            let mut map = HashMap::new();
                            map.insert(String::from("name"), table_name.into());
                            map
                        };
                        context.insert("title", &model_class.as_str());
                        context.insert("base_url", format!("/{}", link_name).as_str());
                        context.insert("description", &LOCALES.lookup_with_args(&locale, "crud_list_description", &args).to_string());
                        context.insert("datas", &results);
                        context.insert("total_pages", &pagination.total_pages);
                        context.insert("current_page", &pagination.current_page);
                        context.insert("current_page_string", &pagination.current_page.to_string());
                        context.insert("offset", &pagination.offset);
                        context.insert("per_page", &pagination.per_page);
                        context.insert("page_numbers", &pagination.generate_page_numbers());
                        context.insert("locale", &locale.to_string());
                        let tera: &mut tera::Tera = &mut config.template.clone();
                        let template_name = $view::index_view(tera);                   
                        let rendered = tera.render(&template_name.as_str(), &context);
                        render_html!(config, rendered)
                    }
                },
                Err(err) => {
                    error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string()).into_response()
                }
            }
        }
    }
}

#[macro_export]
macro_rules! show {
    ($resource:ident, $view:ident) => {
        pub async fn show(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Query(request_query): Query<RequestQuery>, Path(param_id): Path<i32>, State(config): State<Config>) -> impl IntoResponse {
            let tera: &mut Tera = &mut config.template.clone();
            let table_name = stringify!($resource);
            let locale = get_locale(headers, Some(request_query));
            let model_class = table_name.to_class_case();
            match $resource.find(param_id).first::<CrudModel>(&mut config.database.pool.get().unwrap()) {
                Ok(result) => {
                    let mut context = prepare_tera_context(current_user).await;
                    context.insert("data", &result);
                    context.insert("title", &model_class.as_str());
                    context.insert("locale", &"en");
                    context.insert("locale", &locale.to_string());
                    let args = {
                        let mut map = HashMap::new();
                        map.insert(String::from("name"), model_class.into());
                        map
                    };
                    context.insert("description", &"tes test");
                    context.insert("description", &LOCALES.lookup_with_args(&locale, "crud_show_description", &args).to_string());
                    let template_name = $view::show_view(tera);
                    let rendered = tera.render(&template_name.as_str(), &context).unwrap();
                    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
                },
                _ => {
                    error_controller::render_404(config)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! new {
    ($resource:ident, $view:ident) => {
        pub async fn new(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Query(_request_query): Query<RequestQuery>, State(config): State<Config>) -> impl IntoResponse {
            let table_name = stringify!($resource);
            let link_name = table_name.to_kebab_case();
            let form = CrudModel::build_create_form(&config.clone(), headers, format!("/{}", link_name).as_str());
            render_form!(form, $view, config, current_user, None::<Option<ValidationErrors>>)
        }
    }
}

#[macro_export]
macro_rules! edit {
    ($resource:ident, $view:ident) => {
        pub async fn edit(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Query(request_query): Query<RequestQuery>, Path(param_id): Path<i32>, State(config): State<Config>) -> impl IntoResponse {
            let locale = get_locale(headers.clone(), Some(request_query));
            let result = $resource
                .find(param_id)
                .first::<CrudModel>(&mut config.database.pool.get().unwrap())
                .expect(&LOCALES.lookup(&locale, "error_load").to_string());
            let table_name = stringify!($resource);
            let link_name = table_name.to_kebab_case();
            let form = result.build_edit_form(&config.clone(), headers, format!("/{}/{}", link_name, param_id).as_str());
            render_form!(form, $view, config, current_user, None::<Option<ValidationErrors>>)
        }
    }
}

#[macro_export]
macro_rules! delete {
    ($resource:ident) => {
        pub async fn delete(Path(param_id): Path<i32>, headers: HeaderMap, State(config): State<Config>) -> Redirect {
            let table_name = stringify!($resource);
            let locale = get_locale(headers, None);
            let link_name = table_name.to_kebab_case();
            diesel::delete($resource)
                .filter(id.eq(param_id))
                .execute(&mut config.database.pool.get().unwrap())
                .expect(&LOCALES.lookup(&locale, "error_delete").to_string());
            Redirect::to(format!("/{}", link_name).as_str()) 
        }
    }
}

#[macro_export]
macro_rules! render_form {
    ($form:ident, $view:ident, $config:ident, $current_user:ident, $error:expr) => {
        {
            let tera: &mut Tera = &mut $config.template.clone();
            let mut context = prepare_tera_context($current_user).await;
            if let Some(error) = $error {
                let serialized = serde_json::to_string(&error).unwrap();
                context.insert("errors_message", &serialized);
            }
            context.insert("form",&$form);
            let template_name = $view::form_view(tera);
            let rendered = tera.render(&template_name, &context).unwrap();
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}.into_response()
        }  
    };
}

#[macro_export]
macro_rules! render_html {
    ($config:ident, $rendered:ident) => {
        {
            match $rendered {
                Ok(result) => {
                    Response{status_code: axum::http::StatusCode::OK, content_type: "text/html", datas: result}.into_response()
                },
                Err(err) => {
                    error_controller::handler_error($config, axum::http::StatusCode::BAD_REQUEST, err.to_string()).into_response()
                }
            }
        }  
    };
}

#[macro_export]
macro_rules! render_json {
    ($status_code:expr, $results:ident) => {
        {
            match serde_json::to_string(&$results) {
                Ok(serialized) => {
                    Response{status_code: $status_code, content_type: "application/json", datas: serialized}.into_response()
                },
                Err(err) => {
                    Response{status_code: axum::http::StatusCode::BAD_REQUEST, content_type: "application/json", datas: err.to_string()}.into_response()
                }
            }
        }  
    };
}

#[macro_export]
macro_rules! get_total {
    ($config:ident, $model:ident) => {
        {
            let model_name = stringify!($model);
            match $model.count().get_result(&mut $config.database.pool.get().unwrap()) {
                Ok(count) => count,
                Err(e) => {
                    eprintln!("Error counting {}: {}", model_name, e);
                    0 
                }
            }
        }  
    };
}

pub mod api;
pub mod index_controller;
pub mod error_controller;
