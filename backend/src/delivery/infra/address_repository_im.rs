use crate::{
    delivery::domain::{repository::AddressTrait, Address},
    IMPool,
};

use std::sync::Mutex;

struct AddressRepository {
    list: Vec<Address>,
    last_id: i32,
}

lazy_static! {
    static ref DATA: Mutex<AddressRepository> = {
        let mut address1 = Address::new(
            "ul. Piękna 1a".to_string(),
            "Wrocław".to_string(),
            "50-100".to_string(),
        );
        address1.id = 1;
        let mut address2 = Address::new(
            "ul. Złota 12".to_string(),
            "Warszawa".to_string(),
            "01-200".to_string(),
        );
        address2.id = 2;
        let mut address3 = Address::new(
            "ul. Kasztanowa 12".to_string(),
            "Warszawa".to_string(),
            "01-200".to_string(),
        );
        address3.id = 3;
        let mut address4 = Address::new(
            "ul. Kasztanowa 12".to_string(),
            "Wrocław".to_string(),
            "01-200".to_string(),
        );
        address4.id = 4;
        Mutex::new(AddressRepository {
            list: vec![address1, address2, address3, address4],
            last_id: 10,
        })
    };
}

impl AddressTrait<IMPool> for Address {
    fn insert(_: IMPool, address: Address) -> Option<i32> {
        let mut address = address;
        let id = DATA.lock().unwrap().last_id;
        address.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(address);
        Some(id)
    }

    fn delete(_: IMPool, id: i32) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|address: &Address| address.id != id);
        true
    }

    fn save(db_pool: IMPool, address: Address) -> bool {
        Address::delete(db_pool, address.id);
        DATA.lock().unwrap().list.push(address);
        true
    }

    fn find_by_id(_: IMPool, id: i32) -> Option<Address> {
        let list = &DATA.lock().unwrap().list;

        let list = list
            .into_iter()
            .filter(|address| address.id == id)
            .collect::<Vec<&Address>>();
        match list.len() {
            0 => None,
            _ => Some(list.first().unwrap().clone().clone()),
        }
    }

    fn get_all(_: IMPool) -> Vec<Address> {
        DATA.lock().unwrap().list.clone()
    }
}
