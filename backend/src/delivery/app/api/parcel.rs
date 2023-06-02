use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::delivery::app::api::structs::{AddressBody, AddressRequired};
use crate::delivery::domain::value_objects::{ParcelSize, ParcelStatus};

mod views;

mod structs;
use structs::{ModifyParcelRequest, ParcelBody, ParcelRequest, StatusBody};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Courier",
        description = "API for Administration App",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(views::courier_add_status),
    components(schemas(ParcelStatus)),
    tags( (name = "Parcel Status", description = "Parcel Status management"))
)]
struct ApiDocAdmin;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Trade Parner Informations",
        description = "API for Administration App and API for Trade Parnter Integration",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(views::get_parcel, views::add_parcel, views::modify_parcel),
    components(schemas(
        AddressBody,
        ParcelBody,
        ParcelSize,
        ParcelRequest,
        ParcelStatus,
        StatusBody,
        ModifyParcelRequest,
        AddressRequired
    )),
    tags(
        (name = "Trade Partner API", description = "Api for Trade Partner Integration"),
        (name = "Free Access", description = "Inforamtion without authenitcation"),
    )
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
        .service(views::courier_add_status)
        .service(views::modify_parcel);
}
