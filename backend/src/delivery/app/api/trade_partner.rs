use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::delivery::domain::{value_objects::ParcelSize, Address};

mod admin_views;
mod trade_partner_views;

mod structs;
use structs::{
    AddResponse, AddressBody, MoneyBody, PriceListBody, TradePartnerAdminBody, TradePartnerBody,
    TradePartnerListBody, WarehouseAdminBody, WarehouseBody,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        admin_views::add,
        admin_views::get_trade_partner_list,
        admin_views::get_trade_partner,
        admin_views::modify_by_id,
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
        AddResponse,
        PriceListBody,
        ParcelSize,
        TradePartnerListBody,
        Address,
        WarehouseAdminBody,
        WarehouseBody,
        AddressBody
    ))
)]
struct ApiDocAdmin;

#[derive(OpenApi)]
#[openapi(
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
        PriceListBody,
        ParcelSize,
        WarehouseBody,
        AddressBody
    ))
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
        .service(admin_views::add)
        .service(admin_views::modify_by_id)
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
