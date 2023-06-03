#[derive(Clone)]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
    pub trade_partner_id: i32,
    pub address_id: i32,
}

impl Warehouse {
    pub fn new(name: String, trade_partner_id: i32, address_id: i32) -> Self {
        Warehouse {
            id: 0,
            name,
            trade_partner_id,
            address_id,
        }
    }
}
