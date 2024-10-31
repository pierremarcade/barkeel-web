use barkeel_lib::app::Config;
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};
use barkeel_derives::FormBuilder;
use validator::Validate;
use crate::db::schema::users::dsl::*;
use crate::config::constants::USER_COOKIE_NAME;
use crate::app::models::user::User;
use axum::{ 
    http::StatusCode,  
    body::Body,
    response::Response
};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Validate, Clone, FormBuilder)]
#[form_builder(configuration(action_button="login"))]
pub struct Credentials {
    pub email: String,
    pub password: String,
    #[form_builder(label="remember_me")]
    pub remember: bool,
}

/// AuthState is a structure designed to manage authentication state.
///
/// This structure provides a secure way to store and retrieve authentication state,
/// including the authentication token, the currently authenticated user, and the authentication configuration.
/// It uses an optional tuple to manage these pieces of information, and an Arc<Mutex<HashMap<String, String>>> for the configuration,
/// ensuring secure and concurrent access to the authentication state.
///
///
/// ```
#[derive(Clone)]
pub struct AuthState(pub Option<(String, Option<User>, Config)>);

impl AuthState {
    pub async fn get_user(&mut self) -> Option<&User> {
        let (session_tok, store, config) = self.0.as_mut()?;
        if store.is_none() {
            let user = users
                .filter(session_token.eq(session_tok.clone()))
                .first::<User>(&mut config.database.pool.get().unwrap());

            match user {
                Ok(user) => *store = Some(user),
                Err(_e) => {
                    return None;
                }
            }
        }
        store.as_ref()
    }

    pub fn redirect_to_login(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", "/login")
            .header("Set-Cookie", format!("{}=; Max-Age=0", USER_COOKIE_NAME))
            .body(Body::empty())
            .unwrap()
    }
}