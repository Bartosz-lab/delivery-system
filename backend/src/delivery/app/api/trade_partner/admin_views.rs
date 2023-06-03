use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use rust_decimal::{prelude::*, Decimal};
use rusty_money::{iso, Money};

use crate::{
    auth::app::{AdminExtractor, AuthExtractor},
    delivery::{
        app::api::trade_partner::{
            gets,
            structs::{MoneyBody, TradePartnerAdminBody, TradePartnerBody, WarehouseBody},
        },
        domain::{
            repository::{AddressTrait, TradePartnerTrait, WarehouseTrait},
            value_objects::ParcelSize,
            {Address, TradePartner, Warehouse},
        },
    },
    IMPool,
};

type Pool = IMPool;

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Trade Partner",
    request_body(content = TradePartnerBody,
        content_type = "application/json", 
        description = "Create new Trade Partner",
    ),
    responses(
        (status = CREATED, body = i32, description = "Trade Partner created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Trade Partner not created due to invalid data"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("")]
async fn add_trade_parnter(
    db_pool: web::Data<Pool>,
    body: web::Json<TradePartnerBody>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let res = TradePartner::insert((**db_pool).clone(), TradePartner::new(body.name.to_owned()));
    match res {
        Some(id) => HttpResponse::Created().json(id),
        None => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Trade Partner",
    responses(
        (status = OK, body = Vec<TradePartnerAdminBody>, description = "Trade Partner list", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/list")]
async fn get_trade_partner_list(
    db_pool: web::Data<Pool>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let list = TradePartner::get_all((**db_pool).clone());
    HttpResponse::Ok().json(
        list.into_iter()
            .map(|trade_partner| TradePartnerAdminBody {
                id: trade_partner.id,
                name: trade_partner.name,
            })
            .collect::<Vec<TradePartnerAdminBody>>(),
    )
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Trade Partner",
    responses(
        (status = OK, body = TradePartnerBody, description = "Trade Partner data", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}")]
async fn get_trade_partner(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();
    match TradePartner::find_by_id((**db_pool).clone(), trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(trade_partner) => HttpResponse::Ok().json(gets::get_trade_partner(trade_partner)),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Trade Partner",
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
async fn modify_trade_partner(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
    body: web::Json<TradePartnerBody>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();
    match TradePartner::find_by_id((**db_pool).clone(), trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(mut trade_partner) => {
            // There should be data validation
            if body.name != "_" {
                trade_partner.name = body.name.clone()
            }
            if TradePartner::save((**db_pool).clone(), trade_partner) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Trade Partner",
    responses(
        (status = OK, description = "Trade Partner deleted successfully"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{trade_partner_id}")]
async fn delete_trade_partner(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();

    if TradePartner::delete((**db_pool).clone(), trade_partner_id) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Price List",
    responses(
        (status = OK, body = Vec<(ParcelSize, MoneyBody)>, description = "Price List for Trade Partner", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}/pricelist")]
async fn get_price_list(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();
    match TradePartner::find_by_id((**db_pool).clone(), trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(trade_partner) => HttpResponse::Ok().json(gets::get_price_list(trade_partner)),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Price List",
    responses(
        (status = OK, body = MoneyBody, description = "Price of specified size", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Partner or price don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}/pricelist/{size}")]
async fn get_price(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, ParcelSize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, size) = path.into_inner();

    match TradePartner::find_by_id((**db_pool).clone(), trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(trade_partner) => gets::get_price(trade_partner, size),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Price List",
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
    db_pool: web::Data<Pool>,
    body: web::Json<MoneyBody>,
    path: web::Path<(i32, ParcelSize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, size) = path.into_inner();

    match TradePartner::find_by_id((**db_pool).clone(), trade_partner_id) {
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
                    if TradePartner::save((**db_pool).clone(), trade_partner) {
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
    tag = "Price List",
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
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, ParcelSize)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, size) = path.into_inner();

    match TradePartner::find_by_id((**db_pool).clone(), trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(mut trade_partner) => {
            trade_partner.price_list.delete(size);
            if TradePartner::save((**db_pool).clone(), trade_partner) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse",
    responses(
        (status = OK, body = Vec<WarehouseBody>, description = "Warehouse List for Trade Partner", content_type = "application/json"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}/warehouse")]
async fn get_warehouse_list(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let trade_partner_id = path.into_inner();
    match TradePartner::find_by_id((**db_pool).clone(), trade_partner_id) {
        None => HttpResponse::NotFound().finish(),
        Some(_) => HttpResponse::Ok().json(gets::get_warehouse_list(
            (**db_pool).clone(),
            trade_partner_id,
        )),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse",
    responses(
        (status = OK, body = WarehouseAdminBody, description = "Warehouse data", content_type = "application/json"),
        (status = NOT_FOUND, description = "Warehouse don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{trade_partner_id}/warehouse/{warehouse_id}")]
async fn get_warehouse(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, warehouse_id) = path.into_inner();
    gets::get_warehouse((**db_pool).clone(), trade_partner_id, warehouse_id)
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse",
    request_body(content = WarehouseBody,
        content_type = "application/json",
        description = "Add new Warehouse for Trade Partner",
    ),
    responses(
        (status = CREATED,  body = i32, description = "Warehouse created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Warehouse can't be created due to invalid data"),
        (status = NOT_FOUND, description = "Trade Partner don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("/{trade_partner_id}/warehouse")]
async fn add_warehouse(
    db_pool: web::Data<Pool>,
    body: web::Json<WarehouseBody>,
    path: web::Path<i32>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    fn add(
        db_pool: Pool,
        trade_partner_id: i32,
        name: String,
        street: String,
        city: String,
        postal_code: String,
    ) -> HttpResponse {
        match Address::insert(
            db_pool,
            Address::new(street.clone(), city.clone(), postal_code.clone()),
        ) {
            None => HttpResponse::BadRequest().finish(),
            Some(address_id) => {
                match Warehouse::insert(
                    db_pool,
                    Warehouse::new(name.clone(), trade_partner_id, address_id),
                ) {
                    None => HttpResponse::BadRequest().finish(),
                    Some(id) => HttpResponse::Created().json(id),
                }
            }
        }
    }

    let trade_partner_id = path.into_inner();

    match (body.name.clone(), body.address.clone()) {
        (Some(name), Some(address)) => match (address.street, address.city, address.postal_code) {
            (Some(street), Some(city), Some(postal_code)) => add(
                (**db_pool).clone(),
                trade_partner_id,
                name,
                street,
                city,
                postal_code,
            ),
            (_, _, _) => HttpResponse::BadRequest().finish(),
        },
        (_, _) => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse",
    request_body(content = WarehouseBody,
        content_type = "application/json",
        description = "Modify Warehouse for Trade Partner",
    ),
    responses(
        (status = OK,  description = "Warehouse updated successfully"),
        (status = BAD_REQUEST, description = "Warehouse can't be updated due to invalid data"),
        (status = NOT_FOUND, description = "Warehouse don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[put("/{trade_partner_id}/warehouse/{warehouse_id}")]
async fn modify_warehouse(
    db_pool: web::Data<Pool>,
    body: web::Json<WarehouseBody>,
    path: web::Path<(i32, i32)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, warehouse_id) = path.into_inner();

    if let Some((_, mut warehouse)) =
        Warehouse::find_by_trade_partner((**db_pool).clone(), trade_partner_id)
            .into_iter()
            .enumerate()
            .filter(|(id, _)| *id as i32 == warehouse_id)
            .next()
    {
        if let Some(body_address) = &body.address {
            let address = Address::find_by_id((**db_pool).clone(), warehouse.address_id);
            if address.is_none() {
                return HttpResponse::InternalServerError().finish();
            }
            let mut address = address.unwrap();

            if let Some(street) = &body_address.street {
                address.street = street.clone()
            }
            if let Some(city) = &body_address.city {
                address.city = city.clone()
            }
            if let Some(postal_code) = &body_address.postal_code {
                address.postal_code = postal_code.clone()
            }
            if !Address::save((**db_pool).clone(), address) {
                return HttpResponse::BadRequest().finish();
            }
        }
        if let Some(name) = &body.name {
            warehouse.name = name.clone()
        }
        if Warehouse::save((**db_pool).clone(), warehouse) {
            HttpResponse::Ok().finish()
        } else {
            HttpResponse::BadRequest().finish()
        }
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[utoipa::path(
    context_path = "/tradepartner",
    tag = "Warehouse",
    responses(
        (status = OK, description = "Warehouse deleted successfully"),
        (status = NOT_FOUND, description = "Warehouse don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{trade_partner_id}/warehouse/{warehouse_id}")]
async fn delete_warehouse(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (trade_partner_id, warehouse_id) = path.into_inner();

    let warehouse_opt = Warehouse::find_by_trade_partner((**db_pool).clone(), trade_partner_id)
        .into_iter()
        .enumerate()
        .filter(|(id, _)| *id as i32 == warehouse_id)
        .next();
    match warehouse_opt {
        None => HttpResponse::NotFound().finish(),
        Some((_, warehouse)) => {
            Address::delete((**db_pool).clone(), warehouse.address_id);

            if Warehouse::delete((**db_pool).clone(), warehouse.id) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
    }
}
