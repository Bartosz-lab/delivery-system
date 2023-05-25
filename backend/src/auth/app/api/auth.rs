use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web,
    web::Json,
    HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use utoipa::OpenApi;

use crate::auth::app::{AuthExtractor, TokenClaims};
use crate::auth::domain::{repository::UserTrait, User};
use crate::AppState;

mod structs;
use structs::LoginBody;

#[derive(OpenApi)]
#[openapi(paths(login, logout), components(schemas(LoginBody)))]
pub struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(changepass);
}

#[utoipa::path(
    context_path = "/auth",
    request_body(content = LoginBody,
        content_type = "application/json", 
        description = "Login to app",
    ),
    responses(
        (status = ACCEPTED, description = "User was correctly logged"),
        (status = NOT_ACCEPTABLE, description = "Wrong login or password")
    )
)]
#[post("/login")]
async fn login(login_data: Json<LoginBody>, data: web::Data<AppState>) -> impl Responder {
    let user = User::find_by_email(login_data.login.clone());
    match user {
        None => HttpResponse::NotAcceptable().finish(),
        Some(user) => {
            if user.check_password(login_data.password.clone()) {
                let now = Utc::now();
                let claims: TokenClaims = TokenClaims {
                    sub: user.id.to_string(),
                    exp: (now + Duration::minutes(data.env.jwt_expires_in)).timestamp() as usize,
                    iat: now.timestamp() as usize,
                };
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
                )
                .unwrap();
                let cookie = Cookie::build("token", token.to_owned())
                    .path("/")
                    .max_age(ActixWebDuration::minutes(data.env.jwt_maxage))
                    .http_only(true)
                    .finish();

                HttpResponse::Accepted().cookie(cookie).finish()
            } else {
                HttpResponse::NotAcceptable().finish()
            }
        }
    }
}

#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = OK, description = "User has been logged out"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
    )
)]
#[post("/logout")]
async fn logout(_: AuthExtractor) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::minutes(-1))
        .http_only(true)
        .finish();
    HttpResponse::Ok().cookie(cookie).finish()
}

#[get("/changepass")]
async fn changepass() -> impl Responder {
    HttpResponse::Forbidden().finish()
}
