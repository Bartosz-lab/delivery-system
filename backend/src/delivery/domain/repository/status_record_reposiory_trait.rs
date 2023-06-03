use crate::delivery::domain::value_objects::ParcelStatus;
use crate::delivery::domain::StatusRecord;

pub trait StatusRecordTrait<T> {
    fn insert(db_pool: T, address: StatusRecord) -> Option<usize>;

    fn save(db_pool: T, address: StatusRecord) -> bool;

    fn delete(db_pool: T, id: usize) -> bool;

    fn find_by_id(db_pool: T, id: usize) -> Option<StatusRecord>;

    fn find_by_parcel_id(db_pool: T, parcel_id: usize) -> Vec<StatusRecord>;

    fn find_by_status(db_pool: T, status: ParcelStatus) -> Vec<StatusRecord>;
}
