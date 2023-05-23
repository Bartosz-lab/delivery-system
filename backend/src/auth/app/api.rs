use actix_web::{get, http::StatusCode, web, HttpResponse, Responder, Result};
use serde::Serialize;
use utoipa::OpenApi;

use super::super::domain::{role::Role, user::User};

#[derive(OpenApi)]
#[openapi(paths(login))]
pub struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(changepass);
}

#[derive(Serialize)]
enum Status {
    Ok,
    Err,
}

#[derive(Serialize)]
struct LoginResponse {
    status: Status,
    message: String,
}

#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = StatusCode::OK , description = "User was correctly logged", content_type = "application/json")
    )
)]
#[get("/login")]
async fn login() -> Result<impl Responder> {
    let mut user = User::new(
        "Jan".to_string(),
        "Testowy".to_string(),
        "jan@testowy.com".to_string(),
        "123456789".to_string(),
    );
    user.add_role(Role::Admin);
    user.add_role(Role::Courier);
    user.add_role(Role::Admin);
    let obj = LoginResponse {
        status: Status::Ok,
        message: format!("{:?}", user.roles()),
    };
    Ok(web::Json(obj))
}

#[get("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logout page!")
}

#[get("/changepass")]
async fn changepass() -> impl Responder {
    HttpResponse::Ok().status(StatusCode::FORBIDDEN).finish()
}
