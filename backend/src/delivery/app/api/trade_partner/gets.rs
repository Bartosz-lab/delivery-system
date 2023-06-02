use actix_web::HttpResponse;

use crate::delivery::app::api::trade_partner::structs::{
    AddressBody, MoneyBody, TradePartnerBody, WarehouseBody,
};
use crate::delivery::domain::repository::{AddressTrait, WarehouseTrait};
use crate::delivery::domain::value_objects::ParcelSize;
use crate::delivery::domain::{Address, TradePartner, Warehouse};

pub fn get_trade_partner(trade_partner: TradePartner) -> TradePartnerBody {
    TradePartnerBody {
        name: trade_partner.name,
    }
}

pub fn get_price_list(trade_partner: TradePartner) -> Vec<(ParcelSize, MoneyBody)> {
    trade_partner
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
        .collect()
}

pub fn get_price(trade_partner: TradePartner, size: ParcelSize) -> HttpResponse {
    match trade_partner.price_list.get(size) {
        None => HttpResponse::NotFound().finish(),
        Some(price) => HttpResponse::Ok().json(MoneyBody {
            price: price.amount().to_string(),
            currency: price.currency().to_string(),
        }),
    }
}

pub fn get_warehouse_list(trade_partner_id: usize) -> Vec<WarehouseBody> {
    Warehouse::find_by_trade_partner(trade_partner_id)
        .into_iter()
        .enumerate()
        .map(|(id, warehouse)| {
            let address = Address::find_by_id(warehouse.address_id).unwrap();
            WarehouseBody {
                id: Some(id),
                name: Some(warehouse.name),
                address: Some(AddressBody {
                    street: Some(address.street),
                    city: Some(address.city),
                    postal_code: Some(address.postal_code),
                }),
            }
        })
        .collect()
}

pub fn get_warehouse(trade_partner_id: usize, warehouse_id: usize) -> HttpResponse {
    if let Some((_, warehouse)) = Warehouse::find_by_trade_partner(trade_partner_id)
        .into_iter()
        .enumerate()
        .filter(|(id, _)| *id == warehouse_id)
        .next()
    {
        if let Some(address) = Address::find_by_id(warehouse.address_id) {
            HttpResponse::Ok().json(WarehouseBody {
                id: Some(warehouse_id),
                name: Some(warehouse.name),
                address: Some(AddressBody {
                    street: Some(address.street),
                    city: Some(address.city),
                    postal_code: Some(address.postal_code),
                }),
            })
        } else {
            HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::NotFound().finish()
    }
}
