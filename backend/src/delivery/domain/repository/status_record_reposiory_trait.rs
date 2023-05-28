use crate::delivery::domain::StatusRecord;

pub trait StatusRecordTrait {
    fn insert(address: StatusRecord) -> Option<usize>;

    fn save(address: StatusRecord) -> bool;

    fn delete(id: usize) -> bool;

    fn find_by_id(id: usize) -> Option<StatusRecord>;

    fn find_by_parcel_id(parcel_id: usize) -> Vec<StatusRecord>;
}
