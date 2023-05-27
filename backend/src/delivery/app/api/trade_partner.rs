use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::Url;

use crate::delivery::domain::value_objects::ParcelSize;

mod admin_views;

mod structs;
use structs::{AddResponse, MoneyBody, PriceListBody, TradePartnerBody, TradePartnerListBody};

#[derive(OpenApi)]
#[openapi(
    paths(
        admin_views::add,
        admin_views::get_trade_partner_list,
        admin_views::get_trade_partner,
        admin_views::modify_by_id,
        admin_views::delete_trade_parner,
        admin_views::get_price_list,
        admin_views::add_price,
        admin_views::get_price,
        admin_views::delete_price,
    ),
    components(schemas(
        TradePartnerBody,
        MoneyBody,
        AddResponse,
        PriceListBody,
        ParcelSize,
        TradePartnerListBody
    ))
)]
struct ApiDocAdmin;

pub fn swagger_urls() -> Vec<(Url<'static>, utoipa::openapi::OpenApi)> {
    vec![(
        Url::new("Trade Partner Admin", "/api-docs/tradeparner.json"),
        ApiDocAdmin::openapi(),
    )]
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(admin_views::get_trade_partner_list)
        .service(admin_views::get_trade_partner)
        .service(admin_views::add)
        .service(admin_views::modify_by_id)
        .service(admin_views::delete_trade_parner)
        .service(admin_views::get_price_list)
        .service(admin_views::add_price)
        .service(admin_views::get_price)
        .service(admin_views::delete_price);
}
