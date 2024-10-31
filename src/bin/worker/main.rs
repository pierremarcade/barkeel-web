use async_std::sync::Arc;
use barkeel_lib::workers::job::JobService;
use barkeel_lib::workers::redis_service::RedisService;
use barkeel_lib::workers::traits::{JobServiceTrait, RedisServiceTrait};
use barkeel_lib::workers::ThreadPool;
use barkeel_web::config::jobs;
use dotenvy::dotenv;
use log::LevelFilter;
use redis::Client;
use std::env;
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_logger().unwrap();
    jobs::register_jobs().await;

    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let redis_client = Client::open(redis_host).unwrap();
    let redis_service = Arc::new(RedisService::new("my_default", redis_client));
    let job_service = JobService::new(redis_service);
    let thread_pool = ThreadPool::new(Arc::new(job_service));

    let _ = thread_pool.start().await;
}

fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
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
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
