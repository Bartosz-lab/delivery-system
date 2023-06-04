use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    http, post, web,
    web::Json,
    HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::{
    auth::{
        app::{AuthExtractor, ClaimsData, TokenClaims},
        domain::{
            repository::{RoleTrait, UserTrait},
            Role, User,
        },
    },
    AppState, PgPool,
};

mod structs;
use structs::{ChangePassBody, LoginBody};

type Pool = PgPool;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Authenticate",
        description = "API for autenticate in this API",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(login, logout, changepass),
    components(schemas(LoginBody, ChangePassBody)),
    tags(
        (name = "Auth", description = "Authenticate endpoints"),
        (name = "Other", description = "Other endpoints"),
    )
)]
pub struct ApiDoc;

pub fn swagger_urls() -> Vec<(Url<'static>, utoipa::openapi::OpenApi)> {
    vec![(Url::new("Auth", "/api-docs/auth.json"), ApiDoc::openapi())]
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(changepass);
}

#[utoipa::path(
    context_path = "/auth",
    tag = "Auth",
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
async fn login(
    db_pool: web::Data<Pool>,
    body: Json<LoginBody>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user = User::find_by_email((**db_pool).clone(), body.login.clone());
    match user {
        None => HttpResponse::NotAcceptable().finish(),
        Some(user) => {
            if user.check_password(body.password.clone()) {
                let now = Utc::now();
                let claims: TokenClaims = TokenClaims {
                    user_info: ClaimsData {
                        user_id: user.id,
                        roles: Role::get_user_roles((**db_pool).clone(), user.id).unwrap(),
                    },
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

                HttpResponse::Accepted()
                    .cookie(cookie)
                    .append_header((http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
                    .finish()
            } else {
                HttpResponse::NotAcceptable().finish()
            }
        }
    }
}

#[utoipa::path(
    context_path = "/auth",
    tag = "Auth",
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

#[utoipa::path(
    context_path = "/auth",
    tag = "Other",
    request_body(content = ChangePassBody,
        content_type = "application/json", 
        description = "New password",
    ),
    responses(
        (status = OK, description = "Password was changed"),
        (status = NOT_ACCEPTABLE, description = "Password to week"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
    )
)]
#[post("/changepass")]
async fn changepass(
    db_pool: web::Data<Pool>,
    body: Json<ChangePassBody>,
    auth: AuthExtractor,
) -> impl Responder {
    match User::find_by_id((**db_pool).clone(), auth.user.user_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(mut user) => {
            // there should be pass validation
            if true {
                user.set_password(body.password.clone());
                User::save((**db_pool).clone(), user);
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotAcceptable().finish()
            }
        }
    }
}
