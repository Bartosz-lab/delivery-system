use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::delivery::domain::service::parcel_report::structs::{
    AddressBody, DeliveryReport, ParcelBody, ParcelSizeReport, ParcelTotalReport,
    ParcelWarehouseReport,
};
use crate::delivery::domain::service::settlement_report::structs::{
    MoneyBody, SettlementSizeReport, SettlementTotalReport, SettlementWarehouseReport,
};
use crate::delivery::domain::value_objects::ParcelSize;

mod views;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Courier Reports",
        description = "API for Administration App",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(
        views::trade_partner_settlement_report_admin,
        views::collect_report,
        views::delivery_report
    ),
    components(schemas(
        SettlementSizeReport,
        SettlementTotalReport,
        SettlementWarehouseReport,
        MoneyBody,
        ParcelSize,
        ParcelSizeReport,
        ParcelTotalReport,
        ParcelWarehouseReport,
        DeliveryReport,
        ParcelBody,
        AddressBody
    )),
    tags(
        (name = "Settlement Report", description = "Report about Settlement"),
        (name = "Parcel Report", description = "Report about Parcels"),
    )
)]
struct ApiDocAdmin;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Trade Parner Reports",
        description = "API for Administration App",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(views::trade_partner_settlement_report),
    components(schemas(
        SettlementSizeReport,
        SettlementTotalReport,
        SettlementWarehouseReport,
        MoneyBody,
        ParcelSize
    )),
    tags(
        (name = "Settlement Report", description = "Report about Settlement"),
    )
)]
struct ApiDoc;

pub fn swagger_urls() -> Vec<(Url<'static>, utoipa::openapi::OpenApi)> {
    vec![
        (
            Url::new("Reports Admin", "/api-docs/admin/reports.json"),
            ApiDocAdmin::openapi(),
        ),
        (
            Url::new("Reports", "/api-docs/reports.json"),
            ApiDoc::openapi(),
        ),
    ]
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(views::trade_partner_settlement_report)
        .service(views::trade_partner_settlement_report_admin)
        .service(views::collect_report)
        .service(views::delivery_report);
}
