use crate::{
    delivery::domain::{repository::ParcelTrait, value_objects::ParcelSize, Parcel},
    PgPool,
};

use chrono::NaiveDate;
use std::sync::Mutex;

struct ParcelRepository {
    list: Vec<Parcel>,
    last_id: i32,
}

lazy_static! {
    static ref DATA: Mutex<ParcelRepository> = {
        let mut parcel1 = Parcel::new(
            "Jan Kowalski".to_string(),
            "jan.kowalski@example.net".to_string(),
            "123456789".to_string(),
            3,
            3,
            NaiveDate::from_ymd_opt(2023, 5, 28).unwrap(),
            ParcelSize::S,
        );
        parcel1.id = 1;
        let mut parcel2 = Parcel::new(
            "Anna Nowak".to_string(),
            "anna.nowak@example.net".to_string(),
            "123456789".to_string(),
            4,
            3,
            NaiveDate::from_ymd_opt(2023, 5, 28).unwrap(),
            ParcelSize::M,
        );
        parcel2.id = 2;
        Mutex::new(ParcelRepository {
            list: vec![parcel1, parcel2],
            last_id: 10,
        })
    };
}

impl ParcelTrait<PgPool> for Parcel {
    fn insert(_: PgPool, parcel: Parcel) -> Option<i32> {
        let mut parcel = parcel;
        let id = DATA.lock().unwrap().last_id;
        parcel.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().list.push(parcel);
        Some(id)
    }

    fn delete(_: PgPool, id: i32) -> bool {
        let _ = &DATA
            .lock()
            .unwrap()
            .list
            .retain(|parcel: &Parcel| parcel.id != id);
        true
    }

    fn save(db_pool: PgPool, parcel: Parcel) -> bool {
        Parcel::delete(db_pool, parcel.id);
        DATA.lock().unwrap().list.push(parcel);
        true
    }

    fn find_by_id(_: PgPool, id: i32) -> Option<Parcel> {
        let list = &DATA.lock().unwrap().list;

        let list = list
            .into_iter()
            .filter(|parcel| parcel.id == id)
            .collect::<Vec<&Parcel>>();
        match list.len() {
            0 => None,
            _ => Some(list.first().unwrap().clone().clone()),
        }
    }

    fn find_by_warehouse_id(_: PgPool, warehouse_id: i32) -> Vec<Parcel> {
        let list = &DATA.lock().unwrap().list;

        list.into_iter()
            .filter(|parcel| parcel.warehouse_id == warehouse_id)
            .map(|parcel| parcel.clone())
            .collect::<Vec<Parcel>>()
    }

    fn find_by_date_and_warehouse_id(
        _: PgPool,
        start_date: NaiveDate,
        end_date: NaiveDate,
        warehouse_id: i32,
        size: ParcelSize,
    ) -> Vec<Parcel> {
        let list = &DATA.lock().unwrap().list;

        list.into_iter()
            .filter(|parcel| {
                parcel.warehouse_id == warehouse_id
                    && parcel.pickup_date >= start_date
                    && parcel.pickup_date <= end_date
                    && parcel.size == size
            })
            .map(|parcel| parcel.clone())
            .collect::<Vec<Parcel>>()
    }
}
