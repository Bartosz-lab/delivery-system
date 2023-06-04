use chrono::{NaiveDateTime, Utc};

use crate::{delivery::domain::value_objects::ParcelStatus, schema::status_records};

#[derive(Clone, Queryable, Identifiable)]
#[diesel(table_name = status_records)]
pub struct StatusRecord {
    pub id: i32,
    pub parcel_id: i32,
    pub status: ParcelStatus,
    pub creation_time: NaiveDateTime,
}

impl StatusRecord {
    pub fn new(parcel_id: i32, status: ParcelStatus) -> Self {
        StatusRecord {
            id: 0,
            parcel_id,
            status,
            creation_time: Utc::now().naive_utc(),
        }
    }
}
