use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::{
    auth::{
        app::{AdminExtractor, AuthExtractor},
        domain::{repository::RoleTrait, Role},
    },
    IMPool,
};

type Pool = IMPool;

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
async fn view_admin_get_roles(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();
    match Role::get_user_roles(**db_pool, user_id) {
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
async fn view_admin_add_role(
    db_pool: web::Data<Pool>,
    role: web::Json<Role>,
    path: web::Path<i32>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();

    role.attach_user(**db_pool, user_id);
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
async fn view_admin_delete_role(
    db_pool: web::Data<Pool>,
    role: web::Json<Role>,
    path: web::Path<i32>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();

    role.detach_user(**db_pool, user_id);
    HttpResponse::Ok().finish()
}
