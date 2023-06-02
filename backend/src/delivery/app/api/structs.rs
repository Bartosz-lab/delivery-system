use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "Address")]
pub struct AddressBody {
    #[schema(example = "ul. Piękna 1a")]
    pub street: Option<String>,
    #[schema(example = "Wrocław")]
    pub city: Option<String>,
    #[schema(example = "50-100")]
    pub postal_code: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct AddressRequired {
    #[schema(example = "ul. Piękna 1a")]
    pub street: String,
    #[schema(example = "Wrocław")]
    pub city: String,
    #[schema(example = "50-100")]
    pub postal_code: String,
}
