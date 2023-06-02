use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::delivery::app::api::structs::AddressBody;
use crate::delivery::domain::value_objects::ParcelSize;

mod admin_views;
mod gets;
mod trade_partner_views;

mod structs;
use structs::{MoneyBody, TradePartnerAdminBody, TradePartnerBody, WarehouseBody};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Trade Parner Administrator",
        description = "API for Administration App",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(
        admin_views::add_trade_parnter,
        admin_views::get_trade_partner_list,
        admin_views::get_trade_partner,
        admin_views::modify_trade_partner,
        admin_views::delete_trade_partner,
        admin_views::get_price_list,
        admin_views::add_price,
        admin_views::get_price,
        admin_views::delete_price,
        admin_views::get_warehouse_list,
        admin_views::get_warehouse,
        admin_views::add_warehouse,
        admin_views::modify_warehouse,
        admin_views::delete_warehouse,
    ),
    components(schemas(
        TradePartnerBody,
        TradePartnerAdminBody,
        MoneyBody,
        ParcelSize,
        WarehouseBody,
        AddressBody
    )),
    tags(
        (name = "Trade Partner", description = "Trade Partner management"),
        (name = "Price List", description = "Price List management for specific Trade Partner"),
        (name = "Warehouse", description = "Warehouse management for specific Trade Partner"),
    )
)]
struct ApiDocAdmin;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Trade Parner Informations",
        description = "API for Administration App",
        license(name = "MIT"),
        version = "1.0.0"
    ),
    paths(
        trade_partner_views::get_trade_partner,
        trade_partner_views::get_price_list,
        trade_partner_views::get_price,
        trade_partner_views::get_warehouse_list,
        trade_partner_views::get_warehouse,
    ),
    components(schemas(
        TradePartnerBody,
        MoneyBody,
        ParcelSize,
        WarehouseBody,
        AddressBody
    )),
    tags(
        (name = "Trade Partner", description = "Trade Partner Informations"),
        (name = "Price List", description = "Price List Informations"),
        (name = "Warehouse", description = "Warehouse Informations"),
    )
)]
struct ApiDoc;

pub fn swagger_urls() -> Vec<(Url<'static>, utoipa::openapi::OpenApi)> {
    vec![
        (
            Url::new("Trade Partner Admin", "/api-docs/admin/tradepartner.json"),
            ApiDocAdmin::openapi(),
        ),
        (
            Url::new("Trade Partner", "/api-docs/tradepartner.json"),
            ApiDoc::openapi(),
        ),
    ]
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(trade_partner_views::get_trade_partner)
        .service(trade_partner_views::get_price_list)
        .service(trade_partner_views::get_price)
        .service(trade_partner_views::get_warehouse_list)
        .service(trade_partner_views::get_warehouse)
        .service(admin_views::get_trade_partner_list)
        .service(admin_views::get_trade_partner)
        .service(admin_views::add_trade_parnter)
        .service(admin_views::modify_trade_partner)
        .service(admin_views::delete_trade_partner)
        .service(admin_views::get_price_list)
        .service(admin_views::add_price)
        .service(admin_views::get_price)
        .service(admin_views::delete_price)
        .service(admin_views::get_warehouse_list)
        .service(admin_views::get_warehouse)
        .service(admin_views::add_warehouse)
        .service(admin_views::modify_warehouse)
        .service(admin_views::delete_warehouse);
}
