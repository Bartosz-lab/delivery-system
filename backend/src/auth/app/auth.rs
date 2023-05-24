use actix_web::{get, http::StatusCode, post, web, web::Json, HttpResponse, Responder};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(paths(login), components(schemas(LoginBody)))]
pub struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(changepass);
}

#[derive(Deserialize, ToSchema)]
struct LoginBody {
    #[schema(example = "jan@testowy.com")]
    login: String,
    #[schema(example = "password1234")]
    password: String,
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
async fn login(login_data: Json<LoginBody>) -> impl Responder {
    if login_data.login == "jan" && login_data.password == "1234" {
        HttpResponse::Ok().status(StatusCode::ACCEPTED).finish()
    } else {
        HttpResponse::Ok()
            .status(StatusCode::NOT_ACCEPTABLE)
            .finish()
    }
}

#[get("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logout page!")
}

#[get("/changepass")]
async fn changepass() -> impl Responder {
    HttpResponse::Ok().status(StatusCode::FORBIDDEN).finish()
}
