use std::sync::Mutex;

use crate::delivery::domain::{
    repository::StatusRecordTrait, value_objects::ParcelStatus, StatusRecord,
};

struct StatusRecordRepository {
    list: Vec<StatusRecord>,
    last_id: usize,
}

lazy_static! {
    static ref DATA: Mutex<StatusRecordRepository> = {
        let mut status_record1 = StatusRecord::new(1, ParcelStatus::InDelivery);
        status_record1.id = 1;
        let mut status_record2 = StatusRecord::new(1, ParcelStatus::Collected);
        status_record2.id = 2;
        Mutex::new(StatusRecordRepository {
            list: vec![status_record1, status_record2],
            last_id: 10,
        })
    };
}

impl StatusRecordTrait for StatusRecord {
    fn insert(status_record: StatusRecord) -> Option<usize> {
        let mut status_record = status_record;
        let id = DATA.lock().unwrap().last_id;
        status_record.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(status_record);
        Some(id)
    }

    fn delete(id: usize) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|status_record: &StatusRecord| status_record.id != id);
        true
    }

    fn save(status_record: StatusRecord) -> bool {
        StatusRecord::delete(status_record.id);
        DATA.lock().unwrap().list.push(status_record);
        true
    }

    fn find_by_id(id: usize) -> Option<StatusRecord> {
        let list = &DATA.lock().unwrap().list;

        let list = list
            .into_iter()
            .filter(|status_record| status_record.id == id)
            .collect::<Vec<&StatusRecord>>();
        match list.len() {
            0 => None,
            _ => Some(list.first().unwrap().clone().clone()),
        }
    }

    fn find_by_parcel_id(parcel_id: usize) -> Vec<StatusRecord> {
        let list = &DATA.lock().unwrap().list;

        list.into_iter()
            .filter(|status_record| status_record.parcel_id == parcel_id)
            .map(|status_record| status_record.clone())
            .collect::<Vec<StatusRecord>>()
    }
}
