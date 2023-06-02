use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::auth::domain::Role;

mod role_views;
mod user_views;

mod structs;
use structs::{AddUser, UserBody};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Users Administrator",
        description = "API for Administration App",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(
        user_views::view_admin_add_user,
        user_views::view_admin_get_user,
        user_views::view_admin_modify_user,
        user_views::view_admin_delete_user,
        role_views::view_admin_get_roles,
        role_views::view_admin_add_role,
        role_views::view_admin_delete_role,
    ),
    components(schemas(AddUser, UserBody, Role)),
    tags(
        (name = "User", description = "User management"),
        (name = "Role", description = "Role management for specific User"),
    )
)]
pub struct ApiDocAdmin;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "User Informations",
        description = "API for Administration App",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(user_views::view_modify_user, user_views::view_get_user),
    components(schemas(UserBody)),
    tags(
        (name = "User", description = "User management"),
    )
)]
pub struct ApiDoc;

pub fn swagger_urls() -> Vec<(Url<'static>, utoipa::openapi::OpenApi)> {
    vec![
        (
            Url::new("User Admin", "/api-docs/admin/user.json"),
            ApiDocAdmin::openapi(),
        ),
        (Url::new("User", "/api-docs/user.json"), ApiDoc::openapi()),
    ]
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user_views::view_modify_user)
        .service(user_views::view_get_user)
        .service(user_views::view_admin_add_user)
        .service(user_views::view_admin_get_user)
        .service(user_views::view_admin_modify_user)
        .service(user_views::view_admin_delete_user)
        .service(role_views::view_admin_get_roles)
        .service(role_views::view_admin_add_role)
        .service(role_views::view_admin_delete_role);
}
