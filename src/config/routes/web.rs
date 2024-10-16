use axum::{ 
    routing::{ get, post, delete }, 
    Router,
    error_handling::HandleErrorLayer
};
use barkeel_lib::app::Config;
use barkeel_lib::resource_routes;
use crate::app::controllers::*;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

//Add here new route
pub fn routes(_config: Config) -> Router<Config> {
    let public_dir = ServeDir::new("src/public");
    Router::new()
        .route("/", get(index_controller::index))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                .timeout(Duration::from_secs(30))
        )
        .layer(axum::middleware::from_fn(move |req, next| {
            crate::app::middlewares::locale::change_locale(req, next)
        }))
        .nest_service("/public", public_dir.clone()).fallback_service(public_dir)
        .fallback(error_controller::handler_404)
}



