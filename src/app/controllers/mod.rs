use axum::http::{ HeaderMap, header };
use barkeel_lib::session::CSRFManager;
use cookie::Cookie;
use std::sync::Arc;
use tera::Context;
use crate::config::application::Config;

pub fn get_content_type(headers: HeaderMap) -> String {
    let header_value = headers.get("Content-Type");
    let mut content_type = String::new();
    if let Some(header_value) = header_value {
        content_type = header_value.to_str().unwrap().to_owned();
    }
    content_type
}

pub fn is_csrf_token_valid(headers: HeaderMap, config: Arc<Config>, csrf_token: String) -> bool {
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in Cookie::split_parse(cookie_str) {
                let cookie = cookie.unwrap();
                match cookie.name() {
                    "session_token" => {
                        let csrf_manager: &CSRFManager = &config.csrf_manager;
                        if csrf_manager.is_csrf_token_valid(cookie.value().to_string(), csrf_token.clone()) {
                            return true;
                        }
                    },
                    _ =>  {  }
                }
            }
        }
    }
    return false;
}

#[macro_export]
macro_rules! render_html {
    ($config:ident, $rendered:ident) => {
        {
            match $rendered {
                Ok(result) => {
                    Response{status_code: axum::http::StatusCode::OK, content_type: "text/html", datas: result}
                },
                Err(err) => {
                    error_controller::handler_error($config, axum::http::StatusCode::BAD_REQUEST, err.to_string())
                }
            }
        }  
    };
}

#[macro_export]
macro_rules! render_form {
    ($form:ident, $config:ident, $current_user:ident, $error:expr) => {
        {
            let tera: &Tera = &$config.template;
            let mut context = prepare_tera_context($current_user).await;
            if let Some(error) = $error {
                let serialized = serde_json::to_string(&error).unwrap();
                context.insert("errors_message", &serialized);
            }
            context.insert("form",&$form);
            let rendered = tera.render("form.html", &context).unwrap();
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
        }  
    };
}

#[macro_export]
macro_rules! render_json {
    ($config:ident, $results:ident) => {
        {
            match serde_json::to_string(&$results) {
                Ok(serialized) => {
                    return Response{status_code: axum::http::StatusCode::OK, content_type: "application/json", datas: serialized};
                },
                Err(err) => {
                    return error_controller::handler_error($config, axum::http::StatusCode::BAD_REQUEST, err.to_string());
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

pub mod index_controller;
pub mod error_controller;
