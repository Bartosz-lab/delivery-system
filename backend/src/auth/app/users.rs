use actix_web::{http::StatusCode, post, web, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::auth::domain::repository::UserTrait;
use crate::auth::domain::User;

#[derive(OpenApi)]
#[openapi(paths(add), components(schemas(AddBody, AddResponse)))]
pub struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add);
}

#[derive(Deserialize, ToSchema)]
struct AddBody {
    #[schema(example = "Jan")]
    firstname: String,
    #[schema(example = "Testowy")]
    lastname: String,
    #[schema(example = "jan@testowy.com")]
    email: String,
    #[schema(example = "123456789")]
    phone: String,
}

#[derive(Serialize, ToSchema)]
struct AddResponse {
    #[schema(example = 1)]
    id: usize,
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
#[post("/add")]
async fn add(newuser: Json<AddBody>) -> impl Responder {
    let res = User::insert(User::new(
        newuser.firstname.to_owned(),
        newuser.lastname.to_owned(),
        newuser.email.to_owned(),
        newuser.phone.to_owned(),
    ));
    match res {
        Some(id) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .json(AddResponse { id }),
        None => HttpResponse::Ok().status(StatusCode::BAD_REQUEST).finish(),
    }
}
