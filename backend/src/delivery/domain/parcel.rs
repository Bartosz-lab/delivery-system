use chrono::NaiveDate;

use crate::delivery::domain::value_objects::ParcelSize;

#[derive(Clone)]
pub struct Parcel {
    pub id: i32,
    pub recipient_name: String,
    pub recipient_email: String,
    pub recipient_phone: String,
    pub recipient_address_id: i32,
    pub warehouse_id: i32,
    pub pickup_date: NaiveDate,
    pub size: ParcelSize,
}

impl Parcel {
    pub fn new(
        recipient_name: String,
        recipient_email: String,
        recipient_phone: String,
        recipient_address_id: i32,
        warehouse_id: i32,
        pickup_date: NaiveDate,
        size: ParcelSize,
    ) -> Self {
        Parcel {
            id: 0,
            recipient_name,
            recipient_email,
            recipient_phone,
            recipient_address_id,
            warehouse_id,
            pickup_date,
            size,
        }
    }
}
