use actix_web::{get, web, HttpResponse, Responder};

use crate::auth::app::{AuthExtractor, TradePartnerExtractor};
use crate::delivery::app::api::trade_partner::gets;
use crate::delivery::domain::repository::TradePartnerTrait;
use crate::delivery::domain::value_objects::ParcelSize;
use crate::delivery::domain::TradePartner;

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Trade Partner",
    responses(
        (status = OK, body = TradePartnerBody, description = "Trade Partner data", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("")]
async fn get_trade_partner(_: AuthExtractor, extractor: TradePartnerExtractor) -> impl Responder {
    match TradePartner::find_by_id(extractor.trade_partner_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(trade_partner) => HttpResponse::Ok().json(gets::get_trade_partner(trade_partner)),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Price List",
    responses(
        (status = OK, body = Vec<(ParcelSize, MoneyBody)>, description = "Price List for Trade Partner", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/pricelist")]
async fn get_price_list(_: AuthExtractor, extractor: TradePartnerExtractor) -> impl Responder {
    match TradePartner::find_by_id(extractor.trade_partner_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(trade_partner) => HttpResponse::Ok().json(gets::get_price_list(trade_partner)),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Price List",
    params(
        ("size" = ParcelSize, Path, description = "Size of parcel"),
    ),
    responses(
        (status = OK, body = MoneyBody, description = "Price of specified size", content_type = "application/json"),
        (status = NOT_FOUND, description = "Price don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/pricelist/{size}")]
async fn get_price(
    path: web::Path<ParcelSize>,
    _: AuthExtractor,
    extractor: TradePartnerExtractor,
) -> impl Responder {
    let size = path.into_inner();

    match TradePartner::find_by_id(extractor.trade_partner_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(trade_partner) => gets::get_price(trade_partner, size),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse",
    responses(
        (status = OK, body = Vec<WarehouseBody>, description = "Warehouse List for Trade Partner", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/warehouse")]
async fn get_warehouse_list(_: AuthExtractor, extractor: TradePartnerExtractor) -> impl Responder {
    HttpResponse::Ok().json(gets::get_warehouse_list(extractor.trade_partner_id))
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse",
    responses(
        (status = OK, body = WarehouseBody, description = "Warehouse data", content_type = "application/json"),
        (status = NOT_FOUND, description = "Warehouse don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/warehouse/{warehouse_id}")]
async fn get_warehouse(
    path: web::Path<usize>,
    _: AuthExtractor,
    extractor: TradePartnerExtractor,
) -> impl Responder {
    let warehouse_id = path.into_inner();
    gets::get_warehouse(extractor.trade_partner_id, warehouse_id)
}
