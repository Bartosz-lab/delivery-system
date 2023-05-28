use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::delivery::domain::value_objects::{ParcelSize, ParcelStatus};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct StatusBody {
    pub status: ParcelStatus,
    #[schema(example = "28-05-2023 10:38")]
    pub time: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ParcelBody {
    #[schema(example = "Jan Kowalski")]
    pub recipient_name: String,
    #[schema(example = "jan.kowalski@example.net")]
    pub recipient_email: String,
    #[schema(example = "123456789")]
    pub recipient_phone: String,
    pub recipient_address: AddressBody,
    #[schema(example = "28-05-2023")]
    pub pickup_date: String,
    #[schema(example = ParcelSize::L)]
    pub size: ParcelSize,
    pub status_list: Vec<StatusBody>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ParcelRequest {
    #[schema(example = "Jan Kowalski")]
    pub recipient_name: String,
    #[schema(example = "jan.kowalski@example.net")]
    pub recipient_email: String,
    #[schema(example = "123456789")]
    pub recipient_phone: String,
    pub recipient_address: AddressBody,
    #[schema(example = 1)]
    pub warehouse_id: usize,
    #[schema(example = "28-05-2023")]
    pub pickup_date: String,
    #[schema(example = ParcelSize::L)]
    pub size: ParcelSize,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct AddressBody {
    #[schema(example = "ul. Piękna 1a")]
    #[schema(default = "_")]
    pub street: String,
    #[schema(example = "Wrocław")]
    #[schema(default = "_")]
    pub city: String,
    #[schema(example = "50-100")]
    #[schema(default = "_")]
    pub postal_code: String,
}

#[derive(Serialize, ToSchema)]
pub struct AddResponse {
    #[schema(example = 1)]
    pub id: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ModifyParcelRequest {
    pub address: AddressBody,
    #[schema(example = "28-05-2023")]
    pub requested_date: String,
}
