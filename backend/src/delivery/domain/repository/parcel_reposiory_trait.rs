use crate::delivery::domain::Parcel;

pub trait ParcelTrait {
    fn insert(address: Parcel) -> Option<usize>;

    fn save(address: Parcel) -> bool;

    fn delete(id: usize) -> bool;

    fn find_by_id(id: usize) -> Option<Parcel>;

    fn find_by_warehouse_id(warehouse_id: usize) -> Vec<Parcel>;
}
