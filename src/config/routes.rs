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
            
		    // .route("/users", get(user_controller::index))
            // .route("/users/new", get(user_controller::new))
            // .route("/users/:id", get(user_controller::show))
            // .route("/users/:id/edit", get(user_controller::edit))
            // .route("/books/:id", patch(book_controller::update))
            // .route("/books", post(book_controller::create))
            .route("/", get(index_controller::index))
            // .route("/books", get(book_controller::index))
            // .route("/books/new", get(book_controller::new))
            // .route("/books/:id", get(book_controller::show))
            // .route("/books/:id", delete(book_controller::delete))
            // .route("/books/:id/edit", get(book_controller::edit))
            
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                    .timeout(Duration::from_secs(30))
            )
            .route_layer(middleware::from_fn(csrf::unique_id_middleware))
            .fallback(error_controller::handler_404)
            .route("/public/*path", get(index_controller::handle_assets))
}
