use axum::http::Method;
use axum::{extract::DefaultBodyLimit, http::HeaderValue, Router};
use barkeel_lib::app::Config;
use barkeel_lib::session::CSRFManager;
use dotenvy::dotenv;
use fluent_templates::{static_loader, FluentLoader};
use log::LevelFilter;
use std::error::Error;
use std::time::SystemTime;
use tera::Tera;
use tower::layer::Layer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};

#[cfg(feature = "mysql")]
use barkeel_lib::database::mysql::{Connector, Database};
#[cfg(feature = "postgres")]
use barkeel_lib::database::postgres::{Connector, Database};
#[cfg(feature = "sqlite")]
use barkeel_lib::database::sqlite::{Connector, Database};

use crate::config::routes;
static_loader! {
    pub static LOCALES = {
        locales: "src/locales",
        fallback_language: "en",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub struct Loader;

impl Loader {
    pub async fn init() -> Result<(), Box<dyn Error>> {
        // Load environment variables from the .env file
        dotenv().ok();
        // Initialize the logger
        Self::setup_logger()?;
        match Self::check_env_vars() {
            Ok(()) => {
                Self::init_server_web().await?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn init_template() -> Result<Tera, Box<dyn std::error::Error>> {
        let mut tera = Tera::default();
        // Register the Fluent localization function with Tera
        tera.register_function("fluent", FluentLoader::new(&*LOCALES));
        // Add raw templates to Tera from the specified file paths
        tera.add_raw_templates(vec![
            ("base.html", include_str!("../app/views/layouts/base.html")),
            (
                "sidebar.html",
                include_str!("../app/views/layouts/sidebar.html"),
            ),
            (
                "pagination.html",
                include_str!("../app/views/pagination.html"),
            ),
            ("404.html", include_str!("../app/views/errors/404.html")),
            ("error.html", include_str!("../app/views/errors/error.html")),
        ])?;
        Ok(tera)
    }

    async fn init_server_web() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize Tera templates
        let tera = match Self::init_template() {
            Ok(tera) => tera,
            Err(e) => {
                eprintln!("Failed to initialize Tera: {}", e);
                std::process::exit(1);
            }
        };

        // Initialize the database connection
        let database = Self::init_database()?;
        // Create a new CSRF manager instance
        let csrf_manager = CSRFManager::new();
        // Create a configuration object with the database, templates, and CSRF manager
        let config = Config {
            database: database.clone(),
            template: tera,
            csrf_manager,
        };
        // Initialize CORS settings
        let cors = Self::init_cors();
        // Initialize the routes with the configuration
        let routes = routes::web::routes(config.clone());

        let app = NormalizePathLayer::trim_trailing_slash().layer(
            routes
                .with_state(config.clone())
                .layer(cors)
                .layer(DefaultBodyLimit::disable()),
        );

        let host = std::env::var("HOST")?;
        let listener = tokio::net::TcpListener::bind(host).await?;

        axum::serve(
            listener,
            <NormalizePath<Router> as axum::ServiceExt<axum::http::Request<axum::body::Body>>>::into_make_service(
                app
            )
        ).await?;

        Ok(())
    }

    fn init_cors() -> CorsLayer {
        let allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter_map(|s| HeaderValue::from_str(&s).ok())
            .collect::<Vec<_>>();

        (if allowed_origins.is_empty() {
            CorsLayer::new().allow_origin(Any)
        } else {
            CorsLayer::new().allow_origin(AllowOrigin::list(allowed_origins))
        })
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
    }

    pub fn init_database() -> Result<Database, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let pool = Connector::connect(&database_url)?;
        Ok(Database::new(pool))
    }

    fn check_env_vars() -> Result<(), Box<dyn std::error::Error>> {
        let required_vars = vec!["HOST", "DATABASE_URL"];
        for var in required_vars {
            if std::env::var(var).is_err() {
                return Err(format!("{} variable must be defined", var).into());
            }
        }
        Ok(())
    }

    fn log_level() -> Result<LevelFilter, Box<dyn std::error::Error>> {
        match std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .as_str()
        {
            "off" => Ok(LevelFilter::Off),
            "error" => Ok(LevelFilter::Error),
            "warn" => Ok(LevelFilter::Warn),
            "info" => Ok(LevelFilter::Info),
            "debug" => Ok(LevelFilter::Debug),
            "trace" => Ok(LevelFilter::Trace),
            _ => Ok(LevelFilter::Info),
        }
    }

    fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
        let log_level = Self::log_level()?;

        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .level(log_level)
            .chain(std::io::stdout())
            .chain(fern::log_file(
                std::env::var("LOG_PATH")
                    .unwrap_or_else(|_| "output.log".to_string())
                    .as_str(),
            )?)
            .apply()?;
        Ok(())
    }
}
