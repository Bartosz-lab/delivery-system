use crate::delivery::domain::Address;

pub trait AddressTrait<T> {
    fn insert(db_pool: T, address: Address) -> Option<i32>;

    fn save(db_pool: T, address: Address) -> bool;

    fn delete(db_pool: T, id: i32) -> bool;

    fn find_by_id(db_pool: T, id: i32) -> Option<Address>;

    fn get_all(db_pool: T) -> Vec<Address>;
}
