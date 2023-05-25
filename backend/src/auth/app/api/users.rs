use actix_web::web;
use utoipa::OpenApi;

use crate::auth::domain::Role;

mod role_views;
mod user_views;

mod structs;
use structs::{AddResponse, RolesResponse, UserBody};

#[derive(OpenApi)]
#[openapi(
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
    components(schemas(UserBody, AddResponse, RolesResponse, Role))
)]
pub struct ApiDoc;

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
