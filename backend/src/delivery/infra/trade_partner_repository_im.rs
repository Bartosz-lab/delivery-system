use rusty_money::{iso, Money};
use std::sync::Mutex;

use crate::{
    delivery::domain::{
        repository::TradePartnerTrait,
        value_objects::{ParcelSize, PriceList},
        TradePartner,
    },
    IMPool,
};

struct TradePartnerRepository {
    list: Vec<TradePartner>,
    last_id: i32,
}

lazy_static! {
    static ref DATA: Mutex<TradePartnerRepository> = {
        let mut trade_partner1 = TradePartner::new("Y-Kom sp. z o.o.".to_string());
        trade_partner1.id = 1;
        trade_partner1.price_list = PriceList::new();
        trade_partner1
            .price_list
            .add(ParcelSize::S, Money::from_major(1000, iso::PLN));
        trade_partner1
            .price_list
            .add(ParcelSize::M, Money::from_major(2000, iso::PLN));
        trade_partner1
            .price_list
            .add(ParcelSize::L, Money::from_major(3000, iso::PLN));
        let mut trade_partner2 = TradePartner::new("Kompy s.a.".to_string());
        trade_partner2.id = 2;
        Mutex::new(TradePartnerRepository {
            list: vec![trade_partner1, trade_partner2],
            last_id: 10,
        })
    };
}

impl TradePartnerTrait<IMPool> for TradePartner {
    fn insert(_: IMPool, tradepartner: TradePartner) -> Option<i32> {
        let mut tradepartner = tradepartner;
        let id = DATA.lock().unwrap().last_id;
        tradepartner.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(tradepartner);
        Some(id)
    }

    fn delete(_: IMPool, id: i32) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|tradepartner| tradepartner.id != id);
        true
    }

    fn save(db_pool: IMPool, tradepartner: TradePartner) -> bool {
        TradePartner::delete(db_pool, tradepartner.id);
        DATA.lock().unwrap().list.push(tradepartner);
        true
    }

    fn find_by_id(_: IMPool, id: i32) -> Option<TradePartner> {
        let list = &DATA.lock().unwrap().list;

        let list = list
            .into_iter()
            .filter(|tradepartner| tradepartner.id == id)
            .collect::<Vec<&TradePartner>>();
        match list.len() {
            0 => None,
            _ => Some(list.first().unwrap().clone().clone()),
        }
    }

    fn get_all(_: IMPool) -> Vec<TradePartner> {
        DATA.lock().unwrap().list.clone()
    }
}
