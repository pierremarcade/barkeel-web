use dotenvy::dotenv;
use std::env;
use barkeel_web::config::jobs;
use std::sync::Arc;
use barkeel_lib::workers::redis_service::RedisService;
use barkeel_lib::workers::job::JobService;
use barkeel_lib::workers::traits::{ RedisServiceTrait, JobServiceTrait };
use barkeel_lib::workers::ThreadPool;
use redis::Client;

#[tokio::main]
async fn  main() -> ()  {
    dotenv().ok();
    env_logger::init();
    jobs::register_jobs();

    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let redis_client = Client::open(redis_host).unwrap();
    let redis_service = Arc::new(RedisService::new("my_default", redis_client));
    let job_service = JobService::new(redis_service);
    let thread_pool = ThreadPool::new(Arc::new(job_service)); 
    
    let _ = thread_pool.start().await;
}