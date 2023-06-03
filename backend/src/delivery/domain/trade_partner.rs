use crate::{delivery::domain::value_objects::PriceList, schema::trade_partners};

#[derive(Clone, Queryable, Identifiable)]
#[diesel(table_name = trade_partners)]
pub struct TradePartner {
    pub id: i32,
    pub name: String,
    pub price_list: PriceList,
}

impl TradePartner {
    pub fn new(name: String) -> Self {
        TradePartner {
            id: 0,
            name,
            price_list: PriceList::new(),
        }
    }
}
