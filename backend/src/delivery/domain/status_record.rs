use chrono::{DateTime, Utc};

use crate::delivery::domain::value_objects::ParcelStatus;

#[derive(Clone)]
pub struct StatusRecord {
    pub id: usize,
    pub parcel_id: usize,
    pub status: ParcelStatus,
    pub creation_time: DateTime<Utc>,
}

impl StatusRecord {
    pub fn new(parcel_id: usize, status: ParcelStatus) -> Self {
        StatusRecord {
            id: 0,
            parcel_id,
            status,
            creation_time: Utc::now(),
        }
    }
}
