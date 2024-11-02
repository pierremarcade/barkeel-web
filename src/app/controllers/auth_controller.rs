use barkeel_lib::app::Config;
use crate::app::models::user::User;
use crate::app::models::auth::Credentials;
use barkeel_lib::app::http::response::Response;
use crate::db::schema::users::dsl::*;
use tera::{ Context, Tera };
use axum::{
    extract::State,
    response::{ IntoResponse, Response as AxumResponse },
    http::{ HeaderMap, StatusCode },
    Form,
    body::Body,
};
use cookie::{ Cookie, CookieJar, time::Duration };
use barkeel_lib::session::CSRFManager;
use diesel::prelude::*;
use bcrypt::verify;
use crate::config::application::LOCALES;
use fluent_templates::Loader;
use crate::config::constants::{USER_COOKIE_NAME, LOCALE_COOKIE_NAME, AUTH_COOKIE_LIFETIME, AUTH_COOKIE_REMEMBER_ME_LIFETIME};
use crate::app::controllers::DEFAULT_LOCALE;

fn redirect_response(location: &str) -> AxumResponse {
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", location)
        .body(Body::empty())
        .unwrap()
}

fn set_cookie_response(headers: HeaderMap, session_tok: &str, remember: bool) -> AxumResponse {
    let cookie_value = if session_tok.is_empty() {
        format!("{}=; Expires=Thu, 01-Jan-1970 00:00:00 GMT; Max-Age=0", USER_COOKIE_NAME)
    } else {
        let mut jar = CookieJar::new();
        let mut cookie = Cookie::build((USER_COOKIE_NAME, session_tok.to_string()))
            .path("/")
            .http_only(true);

        if let Some(proto) = headers.get("X-Forwarded-Proto") {
            if proto == "https" {
                cookie = cookie.secure(true);
            }
        }

        if remember {
            cookie = cookie.max_age(Duration::days(7));
        } else {
            cookie = cookie.max_age(Duration::hours(2));
        }

        jar.add(cookie);

        jar.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("; ")
    };
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/")
        .header("Set-Cookie", cookie_value)
        .body(Body::empty())
        .unwrap()
}

pub mod get {
    use super::*;
    pub async fn login(headers: HeaderMap, State(config): State<Config>) -> impl IntoResponse {
        let tera: &Tera = &config.template;
        let mut tera = tera.clone();
        tera.add_raw_template("login.html", include_str!("../views/login.html")).unwrap();
        let mut context = Context::new();
        let config_ref = config.clone();
        context.insert("data", &Credentials::build_create_form(&config_ref, headers, "/login"));
        let rendered = tera.render("login.html", &context).unwrap();
        Response { status_code: StatusCode::OK, content_type: "text/html", datas: rendered }
    }

    pub async fn logout(headers: HeaderMap) -> impl IntoResponse {
        set_cookie_response(headers, "", false)
    }
}

pub mod post {
    use super::*;
    pub async fn login(
        State(config): State<Config>,
        headers: HeaderMap,
        Form(creds): Form<Credentials>
    ) -> AxumResponse {
        match
            users
                .filter(email.eq(creds.email))
                .first::<User>(&mut config.database.pool.get().unwrap())
        {
            Ok(user) => {
                match verify(creds.password.clone(), &user.password) {
                    Ok(response) => {
                        if !response {
                            return redirect_response("/login");
                        }
                    }
                    Err(_) => {
                        return redirect_response("/login");
                    }
                }
                let session_tok = new_session(
                    &config,
                    user.id,
                    headers.clone(),
                    creds.remember
                ).await;
                set_cookie_response(headers, &session_tok, creds.remember)
            }
            _ => redirect_response("/login"),
        }
    }
}

pub async fn new_session(
    config: &Config,
    other_user_id: i32,
    headers: HeaderMap,
    remember: bool
) -> String {
    let csrf_manager = CSRFManager::new();
    let locale = barkeel_lib::app::crud::get_locale(
        headers,
        None,
        DEFAULT_LOCALE.to_string(),
        LOCALE_COOKIE_NAME.to_string()
    );
    let exp = expire_session(remember);
    let session_tok = csrf_manager.generate_csrf_token(exp);
    diesel
        ::update(users)
        .filter(id.eq(other_user_id))
        .set(session_token.eq(session_tok.clone()))
        .execute(&mut config.database.pool.get().unwrap())
        .unwrap_or_else(|_| { panic!("{}", LOCALES.lookup(&locale, "error_load")) });

    session_tok
}

fn expire_session(remember: bool) -> usize {
    if remember { AUTH_COOKIE_REMEMBER_ME_LIFETIME } else { AUTH_COOKIE_LIFETIME }
}
