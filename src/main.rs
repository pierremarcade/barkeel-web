pub mod config;
pub mod app;
pub mod db;

use crate::config::application::Loader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Loader::init().await?;
    Ok(())
}
