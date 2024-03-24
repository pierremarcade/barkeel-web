use barkeel_lib::workers::traits::JobTrait;
use barkeel_lib::workers::traits::RedisServiceTrait;
use barkeel_lib::mailer::MailerBuilder;
use barkeel_derives::DelayedJob;
use barkeel_lib::mailer::Email;
use std::error::Error;
use async_trait::async_trait;

#[derive(DelayedJob)]
pub struct Mail;

#[async_trait]
impl JobTrait for Mail {
    async fn perform(&self, _args: Option<Vec<Value>>) -> Result<(), Box<dyn Error>> {
        let email = Email {
            from: "sfsdfsdf".to_string(),
            to: "sdfsdfsdf".to_string(),
            subject: "sdfdsfdsf".to_string(),
            body: "sdgfsdfsdfdsf".to_string(),
        };
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
    }
}