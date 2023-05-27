use crate::delivery::domain::repository::WarehouseTrait;
use crate::delivery::domain::Warehouse;

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

impl WarehouseTrait for Warehouse {
    fn insert(warehouse: Warehouse) -> Option<usize> {
        let mut warehouse = warehouse;
        let id = DATA.lock().unwrap().last_id;
        warehouse.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(warehouse);
        Some(id)
    }

    fn delete(id: usize) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|warehouse| warehouse.id != id);
        true
    }

    fn save(warehouse: Warehouse) -> bool {
        Warehouse::delete(warehouse.id);
        DATA.lock().unwrap().list.push(warehouse);
        true
    }

    fn find_by_id(id: usize) -> Option<Warehouse> {
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

    fn find_by_trade_partner(address_id: usize) -> Vec<Warehouse> {
        let list = &DATA.lock().unwrap().list;

        list.into_iter()
            .filter(|warehouse| warehouse.trade_partner_id == address_id)
            .map(|warehouse| warehouse.clone())
            .collect::<Vec<Warehouse>>()
    }

    fn get_all() -> Vec<Warehouse> {
        DATA.lock().unwrap().list.clone()
    }
}
