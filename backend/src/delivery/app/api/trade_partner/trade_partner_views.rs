use actix_web::{get, web, HttpResponse, Responder};

use crate::auth::app::{AuthExtractor, TradePartnerExtractor};
use crate::delivery::app::api::trade_partner::structs::{
    MoneyBody, PriceListBody, TradePartnerBody,
};
use crate::delivery::domain::repository::TradePartnerTrait;
use crate::delivery::domain::value_objects::ParcelSize;
use crate::delivery::domain::TradePartner;

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Trade Parner",
    responses(
        (status = OK, body = TradePartnerListBody, description = "Trade Parner list", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("")]
async fn get_trade_partner(_: AuthExtractor, extractor: TradePartnerExtractor) -> impl Responder {
    match TradePartner::find_by_id(extractor.trade_partner_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(trade_partner) => HttpResponse::Ok().json(TradePartnerBody {
            name: trade_partner.name,
        }),
    }
}

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Price List",
    responses(
        (status = OK, body = PriceListBody, description = "Price List for Trade Partner", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/pricelist")]
async fn get_price_list(_: AuthExtractor, extractor: TradePartnerExtractor) -> impl Responder {
    match TradePartner::find_by_id(extractor.trade_partner_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(trade_partner) => HttpResponse::Ok().json(PriceListBody {
            list: trade_partner
                .price_list
                .as_vec()
                .into_iter()
                .map(|(size, price)| {
                    (
                        size,
                        MoneyBody {
                            price: price.amount().to_string(),
                            currency: price.currency().to_string(),
                        },
                    )
                })
                .collect(),
        }),
    }
}

#[utoipa::path(
    context_path = "/tradeparner",
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
        Some(trade_partner) => match trade_partner.price_list.get(size) {
            None => HttpResponse::NotFound().finish(),
            Some(price) => HttpResponse::Ok().json(MoneyBody {
                price: price.amount().to_string(),
                currency: price.currency().to_string(),
            }),
        },
    }
}
