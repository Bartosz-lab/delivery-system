use actix_web::{delete, get, post, put, web, web::Json, HttpResponse, Responder};
use rust_decimal::{prelude::*, Decimal};
use rusty_money::{iso, Money};

use crate::auth::app::{AdminExtractor, AuthExtractor};
use crate::delivery::app::api::trade_partner::structs::{
    AddResponse, MoneyBody, PriceListBody, TradePartnerAdminBody, TradePartnerBody,
    TradePartnerListBody, WarehouseBody, WarehouseRequest,
};
use crate::delivery::domain::repository::{AddressTrait, TradePartnerTrait, WarehouseTrait};
use crate::delivery::domain::value_objects::ParcelSize;
use crate::delivery::domain::{Address, TradePartner, Warehouse};

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Trade Partner Admin",
    request_body(content = TradePartnerBody,
        content_type = "application/json", 
        description = "Create new Trade Partner",
    ),
    responses(
        (status = CREATED, body = AddResponse, description = "Trade Partner created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Trade Partner not created due to invalid data"),
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
    context_path = "/tradepartner",
    tag = "Trade Partner Admin",
    responses(
        (status = OK, body = TradePartnerListBody, description = "Trade Partner list", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/list")]
async fn get_trade_partner_list(_: AuthExtractor, _: AdminExtractor) -> impl Responder {
    let list = TradePartner::get_all();
    HttpResponse::Ok().json(TradePartnerListBody {
        list: list
            .into_iter()
            .map(|trade_partner| TradePartnerAdminBody {
                id: trade_partner.id,
                name: trade_partner.name,
            })
            .collect(),
    })
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Trade Partner Admin",
    responses(
        (status = OK, body = TradePartnerBody, description = "Trade Partner data", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
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
    context_path = "/tradepartner",
    tag = "Trade Partner Admin",
    request_body(content = TradePartnerBody,
        content_type = "application/json",
        description = "Modify Trade Partner",
    ),
    responses(
        (status = OK, description = "Trade Partner modified successfully"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
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
    context_path = "/tradepartner",
    tag = "Trade Partner Admin",
    responses(
        (status = OK, description = "Trade Partner deleted successfully"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{trade_partner_id}")]
async fn delete_trade_partner(
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
    context_path = "/tradepartner",
    tag = "Price List Admin",
    responses(
        (status = OK, body = PriceListBody, description = "Price List for Trade Partner", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
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
    context_path = "/tradepartner",
    tag = "Price List Admin",
    responses(
        (status = OK, body = MoneyBody, description = "Price of specified size", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Partner or price don't exist"),
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
    context_path = "/tradepartner",
    tag = "Price List Admin",
    request_body(content = MoneyBody,
        content_type = "application/json", 
        description = "Add/Replace new Price to Price List",
    ),
    responses(
        (status = OK, description = "Price List updated successfully"),
        (status = BAD_REQUEST, description = "Price List can't be updated due to invalid data"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
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
    context_path = "/tradepartner",
    tag = "Price List Admin",
    responses(
        (status = OK, description = "Price deleted successfully"),
        (status = BAD_REQUEST, description = "Price List can't be updated due to invalid data"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
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

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse Admin",
    responses(
        (status = OK, body = Vec<WarehouseBody>, description = "Warehouse List for Trade Partner", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}/warehouse")]
async fn get_warehouse_list(
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();
    match TradePartner::find_by_id(trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(trade_partner) => HttpResponse::Ok().json(
            Warehouse::find_by_trade_partner(trade_partner.id)
                .into_iter()
                .enumerate()
                .map(|(id, warehouse)| WarehouseBody {
                    id,
                    name: warehouse.name,
                    trade_partner_id: warehouse.trade_partner_id,
                    address: Address::find_by_id(warehouse.address_id).unwrap(),
                })
                .collect::<Vec<WarehouseBody>>(),
        ),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse Admin",
    responses(
        (status = OK, body = WarehouseBody, description = "Warehouse data", content_type = "application/json"),
        (status = NOT_FOUND, description = "Warehouse don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}/warehouse/{warehouse_id}")]
async fn get_warehouse(
    path: web::Path<(usize, usize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, warehouse_id) = path.into_inner();

    let warehouse_opt = Warehouse::find_by_trade_partner(trade_partner_id)
        .into_iter()
        .enumerate()
        .filter(|(id, _)| *id == warehouse_id)
        .next();
    match warehouse_opt {
        None => HttpResponse::NotFound().finish(),
        Some((id, warehouse)) => HttpResponse::Ok().json(WarehouseBody {
            id,
            name: warehouse.name,
            trade_partner_id: warehouse.trade_partner_id,
            address: Address::find_by_id(warehouse.address_id).unwrap(),
        }),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse Admin",
    request_body(content = WarehouseRequest,
        content_type = "application/json",
        description = "Add new Warehouse for Trade Partner",
    ),
    responses(
        (status = CREATED,  body = AddResponse, description = "Warehouse created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Warehouse can't be created due to invalid data"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("/{trade_partner_id}/warehouse")]
async fn add_warehouse(
    body: Json<WarehouseRequest>,
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();

    match TradePartner::find_by_id(trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(_) => {
            // There should be data validation
            match Address::insert(Address::new(
                body.address.street.clone(),
                body.address.city.clone(),
                body.address.postal_code.clone(),
            )) {
                None => HttpResponse::BadRequest().finish(),
                Some(address_id) => {
                    match Warehouse::insert(Warehouse::new(
                        body.name.clone(),
                        trade_partner_id,
                        address_id,
                    )) {
                        None => HttpResponse::BadRequest().finish(),
                        Some(id) => HttpResponse::Created().json(AddResponse { id }),
                    }
                }
            }
        }
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse Admin",
    request_body(content = WarehouseRequest,
        content_type = "application/json",
        description = "Modify Warehouse for Trade Partner",
    ),
    responses(
        (status = CREATED,  body = AddResponse, description = "Warehouse updated successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Warehouse can't be updated due to invalid data"),
        (status = NOT_FOUND, description = "Warehouse don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[put("/{trade_partner_id}/warehouse/{warehouse_id}")]
async fn modify_warehouse(
    body: Json<WarehouseRequest>,
    path: web::Path<(usize, usize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, warehouse_id) = path.into_inner();

    let warehouse_opt = Warehouse::find_by_trade_partner(trade_partner_id)
        .into_iter()
        .enumerate()
        .filter(|(id, _)| *id == warehouse_id)
        .next();
    match warehouse_opt {
        None => HttpResponse::NotFound().finish(),
        Some((_, mut warehouse)) => {
            let address = Address::find_by_id(warehouse.address_id);
            if address.is_none() {
                return HttpResponse::InternalServerError().finish();
            }
            let mut address = address.unwrap();

            if body.address.street != "_" {
                address.street = body.address.street.clone()
            }
            if body.address.city != "_" {
                address.city = body.address.city.clone()
            }
            if body.address.postal_code != "_" {
                address.postal_code = body.address.postal_code.clone()
            }
            if !Address::save(address) {
                return HttpResponse::BadRequest().finish();
            }

            if body.name != "_" {
                warehouse.name = body.name.clone()
            }
            if Warehouse::save(warehouse) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse Admin",
    responses(
        (status = OK, description = "Warehouse deleted successfully"),
        (status = NOT_FOUND, description = "Warehouse don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{trade_partner_id}/warehouse/{warehouse_id}")]
async fn delete_warehouse(
    path: web::Path<(usize, usize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, warehouse_id) = path.into_inner();

    let warehouse_opt = Warehouse::find_by_trade_partner(trade_partner_id)
        .into_iter()
        .enumerate()
        .filter(|(id, _)| *id == warehouse_id)
        .next();
    match warehouse_opt {
        None => HttpResponse::NotFound().finish(),
        Some((_, warehouse)) => {
            Address::delete(warehouse.address_id);

            if Warehouse::delete(warehouse.id) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
    }
}
