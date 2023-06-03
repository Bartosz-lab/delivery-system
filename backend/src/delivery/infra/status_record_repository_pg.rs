use std::sync::Mutex;

use crate::{
    delivery::domain::{repository::StatusRecordTrait, value_objects::ParcelStatus, StatusRecord},
    PgPool,
};

struct StatusRecordRepository {
    list: Vec<StatusRecord>,
    last_id: i32,
}

lazy_static! {
    static ref DATA: Mutex<StatusRecordRepository> = {
        let mut status_record1 = StatusRecord::new(1, ParcelStatus::InDelivery);
        status_record1.id = 1;
        let mut status_record2 = StatusRecord::new(1, ParcelStatus::Collected);
        status_record2.id = 2;
        let mut status_record2 =
            StatusRecord::new(1, ParcelStatus::ExpectedDelivery("28-05-2023".to_string()));
        status_record2.id = 3;
        Mutex::new(StatusRecordRepository {
            list: vec![status_record1, status_record2],
            last_id: 10,
        })
    };
}

impl StatusRecordTrait<PgPool> for StatusRecord {
    fn insert(_: PgPool, status_record: StatusRecord) -> Option<i32> {
        let mut status_record = status_record;
        let id = DATA.lock().unwrap().last_id;
        status_record.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(status_record);
        Some(id)
    }

    fn delete(_: PgPool, id: i32) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|status_record: &StatusRecord| status_record.id != id);
        true
    }

    fn save(db_pool: PgPool, status_record: StatusRecord) -> bool {
        StatusRecord::delete(db_pool, status_record.id);
        DATA.lock().unwrap().list.push(status_record);
        true
    }

    fn find_by_id(_: PgPool, id: i32) -> Option<StatusRecord> {
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

    fn find_by_parcel_id(_: PgPool, parcel_id: i32) -> Vec<StatusRecord> {
        let list = &DATA.lock().unwrap().list;

        list.into_iter()
            .filter(|status_record| status_record.parcel_id == parcel_id)
            .map(|status_record| status_record.clone())
            .collect::<Vec<StatusRecord>>()
    }

    fn find_by_status(_: PgPool, status: ParcelStatus) -> Vec<StatusRecord> {
        let list = &DATA.lock().unwrap().list;

        list.into_iter()
            .filter(|status_record| status_record.status == status)
            .map(|status_record| status_record.clone())
            .collect::<Vec<StatusRecord>>()
    }
}
