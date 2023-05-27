use chrono::NaiveDate;

#[derive(Clone)]
pub struct Parcel {
    pub id: usize,
    pub recipient_name: String,
    pub recipient_email: String,
    pub recipient_phone: String,
    pub recipient_address_id: usize,
    pub warehouse_id: usize,
    pub pickup_date: NaiveDate,
}

impl Parcel {
    pub fn new(
        recipient_name: String,
        recipient_email: String,
        recipient_phone: String,
        recipient_address_id: usize,
        warehouse_id: usize,
        pickup_date: NaiveDate,
    ) -> Self {
        Parcel {
            id: 0,
            recipient_name,
            recipient_email,
            recipient_phone,
            recipient_address_id,
            warehouse_id,
            pickup_date,
        }
    }
}
