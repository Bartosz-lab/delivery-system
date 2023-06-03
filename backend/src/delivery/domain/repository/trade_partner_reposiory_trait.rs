use crate::delivery::domain::TradePartner;

pub trait TradePartnerTrait<T> {
    fn insert(db_pool: T, trade_partner: TradePartner) -> Option<i32>;

    fn save(tdb_pool: T, rade_partner: TradePartner) -> bool;

    fn delete(db_pool: T, id: i32) -> bool;

    fn find_by_id(db_pool: T, id: i32) -> Option<TradePartner>;

    fn get_all(db_pool: T) -> Vec<TradePartner>;
}
