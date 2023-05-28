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
    ChangedAddress(usize), //old address id
    Delivered,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, Clone)]
pub enum ParcelSize {
    S,
    M,
    L,
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
}
