use axum::{ 
    routing::{get, post, delete}, 
    Router,
    error_handling::HandleErrorLayer
};
use crate::config::application::Config;
use crate::app::controllers::*;
use std::time::Duration;
use tower::ServiceBuilder;
use inflector::Inflector;

//Add here new route
pub fn routes(config: Config) -> Router<Config> {
    Router::new()
        .route("/", get(index_controller::index))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                .timeout(Duration::from_secs(30))
        )
        .fallback(error_controller::handler_404)
        .route("/public/*path", get(index_controller::handle_assets))
}



