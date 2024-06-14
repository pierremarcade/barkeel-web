use barkeel_lib::workers::traits::JobTrait;
use barkeel_lib::workers::traits::RedisServiceTrait;
use barkeel_lib::mailer::MailerBuilder;
use barkeel_derives::DelayedJob;
use barkeel_lib::mailer::Email;
use std::error::Error;
use async_trait::async_trait;
use serde_json::Map;

#[derive(DelayedJob)]
pub struct Mail;

#[async_trait]
impl JobTrait for Mail {
    async fn perform(&self, args: Option<Vec<Value>>) -> Result<(), Box<dyn Error>> {
        if let Some(args) = args {
            let mut email = Email {
                from: String::from("default@example.com"),
                to: String::from("default@example.com"),
                subject: String::from("Default Subject"),
                body: String::from("Default Body"),
            };
            for arg in args {
                let map: Map<String, Value> = serde_json::from_value(arg).expect("Failed to parse argument");
            
                for (key, value) in map {
                    match key.as_str() {
                        "from" => email.from = value.as_str().unwrap_or("default@example.com").to_string(),
                        "to" => email.to = value.as_str().unwrap_or("default@example.com").to_string(),
                        "subject" => email.subject = value.as_str().unwrap_or("Default Subject").to_string(),
                        "body" => email.body = value.as_str().unwrap_or("Default Body").to_string(),
                        _ => {}
                    }
                }
            }
            
            let mailer = MailerBuilder::new();
            match mailer.send(email).await{
                Ok(_) => {
                    println!("Email sent successfully!");
                    Ok(())
                },
                Err(e) => {
                    println!("Could not send email: {:?}", e);
                    Err(e)
                },
            }
        }else {
            println!("No arguments provided for sending email.");
            Err("No arguments provided".into())
        }
    }
}