use rust_decimal::Decimal;
use rusty_money::{iso, Money};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, Clone)]
pub enum ParcelStatus {
    ToCollect,
    Collected,
    InTransport,
    InTerminal(String),        // Terminal Name
    ExpectedDelivery(String),  // Date as string in format dd-mm-yyyy
    RequestedDelivery(String), // Date as string in format dd-mm-yyyy
    InDelivery,
    ChangedAddress(i32), //old address id
    Delivered,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, Clone, Copy)]
pub enum ParcelSize {
    S,
    M,
    L,
}
impl ParcelSize {
    pub fn iterator() -> impl Iterator<Item = ParcelSize> {
        [ParcelSize::S, ParcelSize::M, ParcelSize::L]
            .iter()
            .copied()
    }
}

#[derive(Clone)]
pub struct PriceList {
    map: HashMap<ParcelSize, Money<'static, iso::Currency>>,
}

impl PriceList {
    pub fn new() -> Self {
        PriceList {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, size: ParcelSize, price: Money<'static, iso::Currency>) -> bool {
        if self.map.contains_key(&size) {
            false
        } else {
            self.map.insert(size, price);
            true
        }
    }

    pub fn replace(&mut self, size: ParcelSize, price: Money<'static, iso::Currency>) {
        if !self.add(size.clone(), price.clone()) {
            if let Some(x) = self.map.get_mut(&size) {
                *x = price;
            }
        }
    }

    pub fn delete(&mut self, size: ParcelSize) {
        self.map.remove(&size);
    }

    pub fn get(&self, size: ParcelSize) -> Option<Money<iso::Currency>> {
        match self.map.get(&size) {
            Some(price) => Some(price.clone()),
            None => None,
        }
    }

    pub fn as_vec(&self) -> Vec<(ParcelSize, Money<iso::Currency>)> {
        let map = self.map.clone();
        map.into_iter().collect()
    }

    pub fn as_ser_vec(&self) -> Vec<(ParcelSize, SerializeMoney)> {
        let map = self.map.clone();
        map.into_iter()
            .map(|(size, money)| {
                (
                    size,
                    SerializeMoney {
                        price: *money.amount(),
                        currency: money.currency().to_string(),
                    },
                )
            })
            .collect()
    }

    pub fn from_ser_vec(list: Vec<(ParcelSize, SerializeMoney)>) -> Self {
        let mut map = HashMap::new();

        for (size, money) in list {
            map.insert(
                size,
                Money::from_decimal(money.price, iso::find(money.currency.as_str()).unwrap()),
            );
        }
        PriceList { map }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SerializeMoney {
    price: Decimal,
    currency: String,
}
