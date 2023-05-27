use actix_web::{delete, get, post, put, web, web::Json, HttpResponse, Responder};
use rust_decimal::{prelude::*, Decimal};
use rusty_money::{iso, Money};

use crate::auth::app::{AdminExtractor, AuthExtractor};
use crate::delivery::app::api::trade_partner::structs::{
    AddResponse, MoneyBody, PriceListBody, TradePartnerBody,
};
use crate::delivery::domain::repository::TradePartnerTrait;
use crate::delivery::domain::value_objects::ParcelSize;
use crate::delivery::domain::TradePartner;

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Trade Parner Admin",
    request_body(content = TradePartnerBody,
        content_type = "application/json", 
        description = "Create new Trade Parner",
    ),
    responses(
        (status = CREATED, body = AddResponse, description = "Trade Parner created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Trade Parner not created due to invalid data"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("")]
async fn add(body: Json<TradePartnerBody>, _: AuthExtractor, _: AdminExtractor) -> impl Responder {
    let res = TradePartner::insert(TradePartner::new(body.name.to_owned()));
    match res {
        Some(id) => HttpResponse::Created().json(AddResponse { id }),
        None => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Trade Parner Admin",
    responses(
        (status = OK, body = TradePartnerBody, description = "Trade Parner data", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Parner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}")]
async fn get_trade_partner(
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();
    match TradePartner::find_by_id(trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(trade_partner) => HttpResponse::Ok().json(TradePartnerBody {
            name: trade_partner.name,
        }),
    }
}

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Trade Parner Admin",
    request_body(content = TradePartnerBody,
        content_type = "application/json",
        description = "Modify Trade Parner",
    ),
    responses(
        (status = OK, description = "Trade Parner modified successfully"),
        (status = NOT_FOUND, description = "Trade Parner don't exist"),
        (status = BAD_REQUEST, description = "Data validation fail"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[put("/{trade_partner_id}")]
async fn modify_by_id(
    path: web::Path<usize>,
    body: Json<TradePartnerBody>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();
    match TradePartner::find_by_id(trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(mut trade_partner) => {
            // There should be data validation
            if body.name != "_" {
                trade_partner.name = body.name.clone()
            }
            if TradePartner::save(trade_partner) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Trade Parner Admin",
    responses(
        (status = OK, description = "Trade Parner deleted successfully"),
        (status = NOT_FOUND, description = "Trade Parner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{trade_partner_id}")]
async fn delete_trade_parner(
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();

    if TradePartner::delete(trade_partner_id) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Price List Admin",
    responses(
        (status = OK, body = PriceListBody, description = "Price List for Trade Partner", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Parner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}/pricelist")]
async fn get_price_list(
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();
    match TradePartner::find_by_id(trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
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
    tag = "Price List Admin",
    responses(
        (status = OK, body = MoneyBody, description = "Price of specified size", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Parner or price don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}/pricelist/{size}")]
async fn get_price(
    path: web::Path<(usize, ParcelSize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, size) = path.into_inner();

    match TradePartner::find_by_id(trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(trade_partner) => match trade_partner.price_list.get(size) {
            None => HttpResponse::NotFound().finish(),
            Some(price) => HttpResponse::Ok().json(MoneyBody {
                price: price.amount().to_string(),
                currency: price.currency().to_string(),
            }),
        },
    }
}

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Price List Admin",
    request_body(content = MoneyBody,
        content_type = "application/json", 
        description = "Add/Replace new Price to Price List",
    ),
    responses(
        (status = OK, description = "Price List updated successfully"),
        (status = BAD_REQUEST, description = "Price List can't be updated due to invalid data"),
        (status = NOT_FOUND, description = "Trade Parner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("/{trade_partner_id}/pricelist/{size}")]
async fn add_price(
    body: Json<MoneyBody>,
    path: web::Path<(usize, ParcelSize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, size) = path.into_inner();

    match TradePartner::find_by_id(trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(mut trade_partner) => {
            // There should be data validation
            let currency = match body.currency.as_str() {
                "PLN" => iso::PLN,
                "EUR" => iso::EUR,
                "USD" => iso::USD,
                _ => return HttpResponse::BadRequest().finish(),
            };
            match Decimal::from_str(body.price.as_str()) {
                Err(_) => HttpResponse::BadRequest().finish(),
                Ok(price) => {
                    trade_partner
                        .price_list
                        .replace(size, Money::from_decimal(price, currency));
                    if TradePartner::save(trade_partner) {
                        HttpResponse::Ok().finish()
                    } else {
                        HttpResponse::BadRequest().finish()
                    }
                }
            }
        }
    }
}

#[utoipa::path(
    context_path = "/tradeparner",
    tag = "Price List Admin",
    responses(
        (status = OK, description = "Price deleted successfully"),
        (status = BAD_REQUEST, description = "Price List can't be updated due to invalid data"),
        (status = NOT_FOUND, description = "Trade Parner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{trade_partner_id}/pricelist/{size}")]
async fn delete_price(
    path: web::Path<(usize, ParcelSize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, size) = path.into_inner();

    match TradePartner::find_by_id(trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(mut trade_partner) => {
            trade_partner.price_list.delete(size);
            if TradePartner::save(trade_partner) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}
