use crate::delivery::domain::repository::AddressTrait;
use crate::delivery::domain::Address;

use std::sync::Mutex;

struct AddressRepository {
    list: Vec<Address>,
    last_id: usize,
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
        Mutex::new(AddressRepository {
            list: vec![address1, address2],
            last_id: 10,
        })
    };
}

impl AddressTrait for Address {
    fn insert(address: Address) -> Option<usize> {
        let mut address = address;
        let id = DATA.lock().unwrap().last_id;
        address.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(address);
        Some(id)
    }

    fn delete(id: usize) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|address: &Address| address.id != id);
        true
    }

    fn save(address: Address) -> bool {
        Address::delete(address.id);
        DATA.lock().unwrap().list.push(address);
        true
    }

    fn find_by_id(id: usize) -> Option<Address> {
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

    fn get_all() -> Vec<Address> {
        DATA.lock().unwrap().list.clone()
    }
}
