use crate::delivery::domain::{value_objects::ParcelSize, Parcel};
use chrono::NaiveDate;

pub trait ParcelTrait {
    fn insert(address: Parcel) -> Option<usize>;

    fn save(address: Parcel) -> bool;

    fn delete(id: usize) -> bool;

    fn find_by_id(id: usize) -> Option<Parcel>;

    fn find_by_warehouse_id(warehouse_id: usize) -> Vec<Parcel>;

    fn find_by_date_and_warehouse_id(
        start_date: NaiveDate,
        end_date: NaiveDate,
        warehouse_id: usize,
        size: ParcelSize,
    ) -> Vec<Parcel>;
}
