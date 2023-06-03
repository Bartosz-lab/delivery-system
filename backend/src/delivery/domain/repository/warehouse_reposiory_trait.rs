use crate::delivery::domain::Warehouse;

pub trait WarehouseTrait<T> {
    fn insert(db_pool: T, warehouse: Warehouse) -> Option<i32>;

    fn save(db_pool: T, warehouse: Warehouse) -> bool;

    fn delete(db_pool: T, id: i32) -> bool;

    fn find_by_id(db_pool: T, id: i32) -> Option<Warehouse>;

    fn find_by_trade_partner(db_pool: T, trade_partner_id: i32) -> Vec<Warehouse>;

    fn find_by_trade_partner_and_id(
        db_pool: T,
        trade_partner_id: i32,
        id: i32,
    ) -> Option<Warehouse>;

    fn get_all(db_pool: T) -> Vec<Warehouse>;
}
