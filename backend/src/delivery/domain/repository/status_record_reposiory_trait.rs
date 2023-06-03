use crate::delivery::domain::value_objects::ParcelStatus;
use crate::delivery::domain::StatusRecord;

pub trait StatusRecordTrait<T> {
    fn insert(db_pool: T, address: StatusRecord) -> Option<i32>;

    fn save(db_pool: T, address: StatusRecord) -> bool;

    fn delete(db_pool: T, id: i32) -> bool;

    fn find_by_id(db_pool: T, id: i32) -> Option<StatusRecord>;

    fn find_by_parcel_id(db_pool: T, parcel_id: i32) -> Vec<StatusRecord>;

    fn find_by_status(db_pool: T, status: ParcelStatus) -> Vec<StatusRecord>;
}
