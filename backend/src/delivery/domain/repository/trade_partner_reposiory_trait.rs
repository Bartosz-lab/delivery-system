use crate::delivery::domain::TradePartner;

pub trait TradePartnerTrait {
    fn insert(trade_partner: TradePartner) -> Option<usize>;

    fn save(trade_partner: TradePartner) -> bool;

    fn delete(id: usize) -> bool;

    fn find_by_id(id: usize) -> Option<TradePartner>;
}
