use crate::delivery::domain::repository::TradePartnerTrait;
use crate::delivery::domain::TradePartner;

use std::sync::Mutex;

struct TradePartnerRepository {
    list: Vec<TradePartner>,
    last_id: usize,
}

lazy_static! {
    static ref DATA: Mutex<TradePartnerRepository> = {
        let mut trade_partner1 = TradePartner::new("Y-Kom sp. z o.o.".to_string());
        trade_partner1.id = 1;
        let mut trade_partner2 = TradePartner::new("Kompy s.a.".to_string());
        trade_partner2.id = 2;
        Mutex::new(TradePartnerRepository {
            list: vec![trade_partner1, trade_partner2],
            last_id: 10,
        })
    };
}

impl TradePartnerTrait for TradePartner {
    fn insert(tradepartner: TradePartner) -> Option<usize> {
        let mut tradepartner = tradepartner;
        let id = DATA.lock().unwrap().last_id;
        tradepartner.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(tradepartner);
        Some(id)
    }

    fn delete(id: usize) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|tradepartner| tradepartner.id != id);
        true
    }

    fn save(tradepartner: TradePartner) -> bool {
        TradePartner::delete(tradepartner.id);
        DATA.lock().unwrap().list.push(tradepartner);
        true
    }

    fn find_by_id(id: usize) -> Option<TradePartner> {
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

    fn get_all() -> Vec<TradePartner> {
        DATA.lock().unwrap().list.clone()
    }
}
