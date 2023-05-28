use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::delivery::domain::value_objects::{ParcelSize, ParcelStatus};

mod views;

mod structs;
use structs::{
    AddResponse, AddressBody, ModifyParcelRequest, ParcelBody, ParcelRequest, StatusBody,
};

#[derive(OpenApi)]
#[openapi(paths(views::add_status), components(schemas(ParcelStatus)))]
struct ApiDocAdmin;

#[derive(OpenApi)]
#[openapi(
    paths(views::get_parcel, views::add_parcel, views::modify_parcel),
    components(schemas(
        AddressBody,
        ParcelBody,
        ParcelSize,
        ParcelRequest,
        AddResponse,
        ParcelStatus,
        StatusBody,
        ModifyParcelRequest
    ))
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
    cfg.service(views::get_parcel)
        .service(views::add_parcel)
        .service(views::add_status)
        .service(views::modify_parcel);
}
