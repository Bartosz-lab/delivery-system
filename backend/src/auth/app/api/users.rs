use actix_web::{http::StatusCode, post, web, web::Json, HttpResponse, Responder};
use utoipa::OpenApi;

use crate::auth::domain::repository::UserTrait;
use crate::auth::domain::User;

mod structs;
use structs::{AddBody, AddResponse};

#[derive(OpenApi)]
#[openapi(paths(add), components(schemas(AddBody, AddResponse)))]
pub struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add);
}

#[utoipa::path(
    context_path = "/user",
    request_body(content = AddBody,
        content_type = "application/json", 
        description = "Create new user",
    ),
    responses(
        (status = CREATED, body = AddResponse, description = "User created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "User not created due to invalid data")
    )
)]
#[post("/")]
async fn add(body: Json<AddBody>) -> impl Responder {
    let res = User::insert(User::new(
        body.firstname.to_owned(),
        body.lastname.to_owned(),
        body.email.to_owned(),
        body.phone.to_owned(),
    ));
    match res {
        Some(id) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .json(AddResponse { id }),
        None => HttpResponse::Ok().status(StatusCode::BAD_REQUEST).finish(),
    }
}
