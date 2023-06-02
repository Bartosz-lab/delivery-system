use actix_web::{delete, get, post, web, web::Json, HttpResponse, Responder};

use crate::auth::app::{AdminExtractor, AuthExtractor};
use crate::auth::domain::repository::RoleTrait;
use crate::auth::domain::Role;

#[utoipa::path(
    context_path = "/user",
    tag = "Role",
    responses(
        (
            status = OK, 
            body = Vec<Role>, 
            description = "User roles", 
            content_type = "application/json", 
            example = json![vec![Role::Admin,  Role::PartnerApi(1)]]
        ),
        (status = NOT_FOUND, description = "User don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{user_id}/role")]
async fn get_user_roles(
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();
    match Role::get_user_roles(user_id) {
        None => HttpResponse::NotFound().finish(),
        Some(roles) => HttpResponse::Ok().json(roles),
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "Role",
    request_body(content = Role,
        content_type = "application/json", 
        description = "Attach role to user",
    ),
    responses(
        (status = OK, description = "Role attached successfully"),
        (status = BAD_REQUEST, description = "Role doesn't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("/{user_id}/role")]
async fn add_role(
    role: Json<Role>,
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();

    role.attach_user(user_id);
    HttpResponse::Ok().finish()
}

#[utoipa::path(
    context_path = "/user",
    tag = "Role",
    request_body(content = Role,
        content_type = "application/json", 
        description = "Detach role from user",
    ),
    responses(
        (status = OK, description = "Role detached successfully"),
        (status = BAD_REQUEST, description = "Role doesn't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{user_id}/role")]
async fn del_role(
    role: Json<Role>,
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();

    role.detach_user(user_id);
    HttpResponse::Ok().finish()
}
