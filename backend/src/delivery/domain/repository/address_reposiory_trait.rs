use crate::delivery::domain::Address;

pub trait AddressTrait {
    fn insert(address: Address) -> Option<usize>;

    fn save(address: Address) -> bool;

    fn delete(id: usize) -> bool;

    fn find_by_id(id: usize) -> Option<Address>;

    fn get_all() -> Vec<Address>;
}
