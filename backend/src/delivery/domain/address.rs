use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::addresses;

#[derive(Clone, Serialize, Deserialize, ToSchema, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = addresses)]
pub struct Address {
    pub id: i32,
    #[schema(example = "ul. Piękna 1a")]
    pub street: String,
    #[schema(example = "Wrocław")]
    pub city: String,
    #[schema(example = "50-100")]
    pub postal_code: String,
}

impl Address {
    pub fn new(street: String, city: String, postal_code: String) -> Self {
        Address {
            id: 0,
            street,
            city,
            postal_code,
        }
    }
}
