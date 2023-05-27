use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct Address {
    pub id: usize,
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
