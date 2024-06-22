use axum::{ 
    routing::{get, post, patch, delete}, 
    Router,
    error_handling::HandleErrorLayer,
    middleware
};
use std::sync::Arc;
use crate::config::application::Config;
use crate::app::controllers::*;
use crate::app::middlewares::*;
use std::time::Duration;
use tower::ServiceBuilder;

//Add here new route
pub fn routes() -> Router<Arc<Config>> {
    Router::new()
            .route("/", get(index_controller::index))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                    .timeout(Duration::from_secs(30))
            )
            .route_layer(middleware::from_fn(csrf::unique_id_middleware))
            .fallback(error_controller::handler_404)
            .route("/public/*path", get(index_controller::handle_assets))
}
