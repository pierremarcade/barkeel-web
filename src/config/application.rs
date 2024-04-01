use dotenvy::dotenv;
use crate::config::routes;
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;
#[cfg(feature = "postgres")]
use barkeel_lib::csrf::CSRFManager;
use crate::config::database::postgres::{Connector, Database};
#[cfg(feature = "mysql")]
use crate::config::database::mysql::{Connector, Database};
#[cfg(feature = "sqlite")]
use crate::config::database::sqlite::{Connector, Database};
use tera::Tera;
use std::error::Error;
use axum::extract::Extension;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: Database,
    pub template: Tera,
    pub csrf_manager: CSRFManager,
}

pub struct Loader;

impl Loader {
    pub async fn init() -> Result<(), Box<dyn Error>> {
        dotenv().ok();
        env_logger::init();
		match Self::check_env_vars() {
			Ok(()) => {
				Self::init_server_web().await?;
				Ok(())
			},
			Err(e) => Err(e),
		}   
    }

    fn init_template() -> Result<Tera, Box<dyn std::error::Error>> {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            ("base.html", include_str!("../app/views/layouts/base.html")),
            ("header.html", include_str!("../app/views/layouts/header.html")),
            ("footer.html", include_str!("../app/views/layouts/footer.html")),
            ("404.html", include_str!("../app/views/errors/404.html")),
            ("error.html", include_str!("../app/views/errors/error.html")),
        ])?;
        Ok(tera)
    }

    async fn init_server_web() -> Result<(), Box<dyn std::error::Error>> {
        let tera = match Self::init_template() {
            Ok(tera) => tera,
            Err(e) => {
                eprintln!("Failed to initialize Tera: {}", e);
                std::process::exit(1);
            }
        };
        let database = Self::init_database()?;
        let csrf_manager = CSRFManager::new();
        let shared_state = Arc::new(Config { database: database.clone(), template: tera, csrf_manager });
        let cors = CorsLayer::new().allow_origin(Any);

        let app = routes::routes(shared_state.clone())
            .with_state(shared_state.clone())
            .layer(Extension(shared_state))
            .layer(cors);
        
        let host = std::env::var("HOST")?;
        let port = std::env::var("PORT")?;
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    pub fn init_database() -> Result<Database, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let pool = Connector::connect(&database_url)?;
        Ok(Database::new(pool))
    }

	fn check_env_vars() -> Result<(), Box<dyn std::error::Error>> {
		let required_vars = vec!["HOST", "PORT", "DATABASE_URL"];
		for var in required_vars {
			if std::env::var(var).is_err() {
				return Err(format!("{} variable must be defined", var).into());
			}
		}
		Ok(())
	}
}