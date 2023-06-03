use crate::delivery::domain::{value_objects::ParcelSize, Parcel};
use chrono::NaiveDate;

pub trait ParcelTrait<T> {
    fn insert(db_pool: T, address: Parcel) -> Option<usize>;

    fn save(db_pool: T, address: Parcel) -> bool;

    fn delete(db_pool: T, id: usize) -> bool;

    fn find_by_id(db_pool: T, id: usize) -> Option<Parcel>;

    fn find_by_warehouse_id(db_pool: T, warehouse_id: usize) -> Vec<Parcel>;

    fn find_by_date_and_warehouse_id(
        db_pool: T,
        start_date: NaiveDate,
        end_date: NaiveDate,
        warehouse_id: usize,
        size: ParcelSize,
    ) -> Vec<Parcel>;
}
