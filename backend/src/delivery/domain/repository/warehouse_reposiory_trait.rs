use crate::delivery::domain::Warehouse;

pub trait WarehouseTrait<T> {
    fn insert(db_pool: T, warehouse: Warehouse) -> Option<usize>;

    fn save(db_pool: T, warehouse: Warehouse) -> bool;

    fn delete(db_pool: T, id: usize) -> bool;

    fn find_by_id(db_pool: T, id: usize) -> Option<Warehouse>;

    fn find_by_trade_partner(db_pool: T, trade_partner_id: usize) -> Vec<Warehouse>;

    fn find_by_trade_partner_and_id(
        db_pool: T,
        trade_partner_id: usize,
        id: usize,
    ) -> Option<Warehouse>;

    fn get_all(db_pool: T) -> Vec<Warehouse>;
}
