use crate::delivery::domain::Warehouse;

pub trait WarehouseTrait {
    fn insert(warehouse: Warehouse) -> Option<usize>;

    fn save(warehouse: Warehouse) -> bool;

    fn delete(id: usize) -> bool;

    fn find_by_id(id: usize) -> Option<Warehouse>;

    fn find_by_trade_partner(trade_partner_id: usize) -> Vec<Warehouse>;

    fn get_all() -> Vec<Warehouse>;
}
