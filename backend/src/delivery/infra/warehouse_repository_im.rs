use crate::{
    delivery::domain::{repository::WarehouseTrait, Warehouse},
    IMPool,
};

use std::sync::Mutex;

struct WarehouseRepository {
    list: Vec<Warehouse>,
    last_id: usize,
}

lazy_static! {
    static ref DATA: Mutex<WarehouseRepository> = {
        let mut address1 = Warehouse::new("Magazyn 1".to_string(), 1, 1);
        address1.id = 3;
        let mut address2 = Warehouse::new("Magazyn 2".to_string(), 1, 2);
        address2.id = 4;
        Mutex::new(WarehouseRepository {
            list: vec![address1, address2],
            last_id: 10,
        })
    };
}

impl WarehouseTrait<IMPool> for Warehouse {
    fn insert(_: IMPool, warehouse: Warehouse) -> Option<usize> {
        let mut warehouse = warehouse;
        let id = DATA.lock().unwrap().last_id;
        warehouse.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(warehouse);
        Some(id)
    }

    fn delete(_: IMPool, id: usize) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|warehouse| warehouse.id != id);
        true
    }

    fn save(db_pool: IMPool, warehouse: Warehouse) -> bool {
        Warehouse::delete(db_pool, warehouse.id);
        DATA.lock().unwrap().list.push(warehouse);
        true
    }

    fn find_by_id(_: IMPool, id: usize) -> Option<Warehouse> {
        let list = &DATA.lock().unwrap().list;

        let list = list
            .into_iter()
            .filter(|warehouse| warehouse.id == id)
            .collect::<Vec<&Warehouse>>();
        match list.len() {
            0 => None,
            _ => Some(list.first().unwrap().clone().clone()),
        }
    }

    fn find_by_trade_partner(_: IMPool, trade_partner_id: usize) -> Vec<Warehouse> {
        let list = &DATA.lock().unwrap().list;

        list.into_iter()
            .filter(|warehouse| warehouse.trade_partner_id == trade_partner_id)
            .enumerate()
            .map(|(id, warehouse)| {
                let mut new = warehouse.clone();
                new.id = id;
                new.clone()
            })
            .collect::<Vec<Warehouse>>()
    }

    fn find_by_trade_partner_and_id(
        _: IMPool,
        trade_partner_id: usize,
        warehouse_id: usize,
    ) -> Option<Warehouse> {
        let list = &DATA.lock().unwrap().list;

        match list
            .into_iter()
            .filter(|warehouse| warehouse.trade_partner_id == trade_partner_id)
            .enumerate()
            .filter(|(id, _)| *id == warehouse_id)
            .next()
        {
            None => None,
            Some((_, warehouse)) => Some(warehouse.clone()),
        }
    }

    fn get_all(_: IMPool) -> Vec<Warehouse> {
        DATA.lock().unwrap().list.clone()
    }
}
