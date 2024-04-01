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
pub fn routes(config: Arc<Config>) -> Router<Arc<Config>> {
    Router::new()
            
            .route_layer(middleware::from_fn(csrf::ensure_csrf_is_valid_middleware))
            .route("/", get(index_controller::index))
		    // .route("/users", get(user_controller::index))
            // .route("/users/new", get(user_controller::new))
            // .route("/users/:id", get(user_controller::show))
            // .route("/users/:id/edit", get(user_controller::edit))
            // .route(
            //     "/articles",
            //     post({
            //         let conflig_clone = config.clone();
            //         move |body| article_controller::create(body, conflig_clone)
            //     }),
            // )
            // .route(
            //     "/articles/:id",
            //     post({
            //         let config_clone = config.clone();
            //         move |id, body| article_controller::update(id, body, config_clone)
            //     }),
            // )
            // .route("/articles", get(article_controller::index))
            // .route("/articles/new", get(article_controller::new))
            // .route("/articles/:id", get(article_controller::show))
            // .route("/articles/:id", delete(article_controller::delete))
            // .route("/articles/:id/edit", get(article_controller::edit))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                    .timeout(Duration::from_secs(30))
            )
            .route_layer(middleware::from_fn(csrf::unique_id_middleware))
            .fallback(error_controller::handler_404)
            .route("/public/*path", get(index_controller::handle_assets))
}
