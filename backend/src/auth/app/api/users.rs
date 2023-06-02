use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::auth::domain::Role;

mod role_views;
mod user_views;

mod structs;
use structs::{AddResponse, UserBody};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Users Administrator",
        description = "API for Administration App",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(
        user_views::add,
        user_views::modify,
        user_views::get_user,
        user_views::get_user_by_id,
        user_views::modify_by_id,
        user_views::delete_user,
        role_views::get_user_roles,
        role_views::add_role,
        role_views::del_role,
    ),
    components(schemas(UserBody, AddResponse, Role)),
    tags(
        (name = "User", description = "User management"),
        (name = "Role", description = "Role management for specific User"),
    )
)]
pub struct ApiDocAdmin;

#[derive(OpenApi)]
#[openapi(
    paths(
        user_views::add,
        user_views::modify,
        user_views::get_user,
        user_views::get_user_by_id,
        user_views::modify_by_id,
        user_views::delete_user,
    ),
    components(schemas(UserBody, AddResponse, Role))
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
    cfg.service(user_views::add)
        .service(user_views::modify)
        .service(user_views::get_user)
        .service(user_views::get_user_by_id)
        .service(user_views::modify_by_id)
        .service(user_views::delete_user)
        .service(role_views::get_user_roles)
        .service(role_views::add_role)
        .service(role_views::del_role);
}
