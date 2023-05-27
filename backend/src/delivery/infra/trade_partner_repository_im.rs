use crate::delivery::domain::repository::TradePartnerTrait;
use crate::delivery::domain::TradePartner;

use std::sync::Mutex;

struct TradeParnerRepository {
    tradeparners: Vec<TradePartner>,
    last_id: usize,
}

lazy_static! {
    static ref DATA: Mutex<TradeParnerRepository> = {
        let mut trade_partner1 = TradePartner::new("Y-Kom sp. z o.o.".to_string());
        trade_partner1.id = 1;
        let mut trade_partner2 = TradePartner::new("Kompy s.a.".to_string());
        trade_partner2.id = 2;
        Mutex::new(TradeParnerRepository {
            tradeparners: vec![trade_partner1, trade_partner2],
            last_id: 10,
        })
    };
}

impl TradePartnerTrait for TradePartner {
    fn insert(tradeparner: TradePartner) -> Option<usize> {
        let mut tradeparner = tradeparner;
        let id = DATA.lock().unwrap().last_id;
        tradeparner.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().tradeparners.push(tradeparner);
        Some(id)
    }

    fn delete(tradeparner_id: usize) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .tradeparners
            .retain(|tradeparner| tradeparner.id != tradeparner_id);
        true
    }

    fn save(tradeparner: TradePartner) -> bool {
        TradePartner::delete(tradeparner.id);
        DATA.lock().unwrap().tradeparners.push(tradeparner);
        true
    }

    fn find_by_id(id: usize) -> Option<TradePartner> {
        let tradeparners = &DATA.lock().unwrap().tradeparners;

        let tradeparners = tradeparners
            .into_iter()
            .filter(|tradeparner| tradeparner.id == id)
            .collect::<Vec<&TradePartner>>();
        match tradeparners.len() {
            0 => None,
            _ => Some(tradeparners.first().unwrap().clone().clone()),
        }
    }

    fn get_all() -> Vec<TradePartner> {
        DATA.lock().unwrap().tradeparners.clone()
    }
}
