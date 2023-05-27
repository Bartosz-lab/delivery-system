use actix_web::{delete, get, post, put, web, web::Json, HttpResponse, Responder};

use crate::auth::app::api::users::structs::{AddResponse, UserBody};
use crate::auth::app::{AdminExtractor, AuthExtractor};
use crate::auth::domain::repository::UserTrait;
use crate::auth::domain::User;

#[utoipa::path(
    context_path = "/user",
    tag = "User Admin",
    request_body(content = UserBody,
        content_type = "application/json", 
        description = "Create new user",
    ),
    responses(
        (status = CREATED, body = AddResponse, description = "User created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "User not created due to invalid data"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("")]
async fn add(body: Json<UserBody>, _: AuthExtractor, _: AdminExtractor) -> impl Responder {
    let res = User::insert(User::new(
        body.firstname.to_owned(),
        body.lastname.to_owned(),
        body.email.to_owned(),
        body.phone.to_owned(),
    ));
    match res {
        Some(id) => HttpResponse::Created().json(AddResponse { id }),
        None => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "User",
    request_body(content = UserBody,
        content_type = "application/json", 
        description = "Modify user",
    ),
    responses(
        (status = CREATED, description = "User modified successfully"),
        (status = BAD_REQUEST, description = "Data validation fail"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
    )
)]
#[put("")]
async fn modify(body: Json<UserBody>, auth: AuthExtractor) -> impl Responder {
    match User::find_by_id(auth.user.user_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(mut user) => {
            // There should be data validation
            if body.firstname != "_" {
                user.firstname = body.firstname.clone()
            }
            if body.lastname != "_" {
                user.lastname = body.lastname.clone()
            }
            if body.email != "_" {
                user.email = body.email.clone()
            }
            if body.phone != "_" {
                user.phone = body.phone.clone()
            }
            if User::save(user) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "User",
    responses(
        (status = OK, body = UserBody, description = "User data", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
    )
)]
#[get("")]
async fn get_user(auth: AuthExtractor) -> impl Responder {
    match User::find_by_id(auth.user.user_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(user) => HttpResponse::Ok().json(UserBody {
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            phone: user.phone,
        }),
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "User Admin",
    responses(
        (status = OK, body = UserBody, description = "User data", content_type = "application/json"),
        (status = NOT_FOUND, description = "User don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{user_id}")]
async fn get_user_by_id(
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();
    match User::find_by_id(user_id) {
        None => HttpResponse::NotFound().finish(),
        Some(user) => HttpResponse::Ok().json(UserBody {
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            phone: user.phone,
        }),
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "User Admin",
    request_body(content = UserBody,
        content_type = "application/json", 
        description = "Modify user",
    ),
    responses(
        (status = CREATED, description = "User modified successfully"),
        (status = NOT_FOUND, description = "User don't exist"),
        (status = BAD_REQUEST, description = "Data validation fail"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[put("/{user_id}")]
async fn modify_by_id(
    path: web::Path<usize>,
    body: Json<UserBody>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();
    match User::find_by_id(user_id) {
        None => HttpResponse::NotFound().finish(),
        Some(mut user) => {
            // There should be data validation
            if body.firstname != "_" {
                user.firstname = body.firstname.clone()
            }
            if body.lastname != "_" {
                user.lastname = body.lastname.clone()
            }
            if body.email != "_" {
                user.email = body.email.clone()
            }
            if body.phone != "_" {
                user.phone = body.phone.clone()
            }
            if User::save(user) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "User Admin",
    responses(
        (status = CREATED, description = "User modified successfully"),
        (status = NOT_FOUND, description = "User don't exist"),
        (status = BAD_REQUEST, description = "User don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{user_id}")]
async fn delete_user(
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();
    match User::find_by_id(user_id) {
        None => HttpResponse::NotFound().finish(),
        Some(user) => {
            if User::save(user) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}
