#[derive(Clone)]
pub struct Warehouse {
    pub id: usize,
    pub name: String,
    pub trade_partner_id: usize,
    pub address_id: usize,
}

impl Warehouse {
    pub fn new(name: String, trade_partner_id: usize, address_id: usize) -> Self {
        Warehouse {
            id: 0,
            name,
            trade_partner_id,
            address_id,
        }
    }
}
