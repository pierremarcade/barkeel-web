use axum::{ routing::get, Router };
use std::sync::Arc;
use crate::config::application::Config;
use crate::app::controllers::api::*;

//Add here new route
pub fn routes(_config: Arc<Config>) -> Router<Arc<Config>> {
    let api_routes = Router::new();
    Router::new().nest("/v1", api_routes)
}



