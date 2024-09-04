use axum::{ routing::get, Router };
use barkeel_lib::app::Config;
use crate::app::controllers::api::*;

//Add here new route
pub fn routes(_config: Config) -> Router<Config> {
    let api_routes = Router::new();
    Router::new().nest("/v1", api_routes)
}



