use actix_web::{get, web, HttpResponse, Responder};

use crate::auth::app::{AuthExtractor, TradePartnerExtractor};
use crate::delivery::app::api::trade_partner::structs::{
    AddressBody, MoneyBody, PriceListBody, TradePartnerBody, WarehouseBody,
};
use crate::delivery::domain::repository::{AddressTrait, TradePartnerTrait, WarehouseTrait};
use crate::delivery::domain::value_objects::ParcelSize;
use crate::delivery::domain::{Address, TradePartner, Warehouse};

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
        Some(trade_partner) => HttpResponse::Ok().json(TradePartnerBody {
            name: trade_partner.name,
        }),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
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
    HttpResponse::Ok().json(
        Warehouse::find_by_trade_partner(extractor.trade_partner_id)
            .into_iter()
            .enumerate()
            .map(|(_, warehouse)| {
                let address = Address::find_by_id(warehouse.address_id).unwrap();
                WarehouseBody {
                    name: warehouse.name,
                    address: AddressBody {
                        street: address.street,
                        city: address.city,
                        postal_code: address.postal_code,
                    },
                }
            })
            .collect::<Vec<WarehouseBody>>(),
    )
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

    let warehouse_opt = Warehouse::find_by_trade_partner(extractor.trade_partner_id)
        .into_iter()
        .enumerate()
        .filter(|(id, _)| *id == warehouse_id)
        .next();
    match warehouse_opt {
        None => HttpResponse::NotFound().finish(),
        Some((_, warehouse)) => {
            let address = Address::find_by_id(warehouse.address_id).unwrap();
            HttpResponse::Ok().json(WarehouseBody {
                name: warehouse.name,
                address: AddressBody {
                    street: address.street,
                    city: address.city,
                    postal_code: address.postal_code,
                },
            })
        }
    }
}
