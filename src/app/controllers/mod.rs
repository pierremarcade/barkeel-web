
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
