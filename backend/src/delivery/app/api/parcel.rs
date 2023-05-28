use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::delivery::domain::value_objects::ParcelSize;

mod views;

mod structs;
use structs::{AddressBody, ParcelBody};

#[derive(OpenApi)]
#[openapi(paths(), components(schemas(AddressBody, ParcelBody, ParcelSize)))]
struct ApiDocAdmin;

#[derive(OpenApi)]
#[openapi(
    paths(views::get_parcel),
    components(schemas(AddressBody, ParcelBody, ParcelSize))
)]
struct ApiDoc;

pub fn swagger_urls() -> Vec<(Url<'static>, utoipa::openapi::OpenApi)> {
    vec![
        (
            Url::new("Parcel Admin", "/api-docs/admin/parcel.json"),
            ApiDocAdmin::openapi(),
        ),
        (
            Url::new("Parcel", "/api-docs/parcel.json"),
            ApiDoc::openapi(),
        ),
    ]
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(views::get_parcel);
}
