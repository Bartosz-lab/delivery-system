use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::delivery::domain::value_objects::ParcelSize;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ParcelTotalReport {
    #[schema(example = "28-05-2023")]
    pub date: String,
    pub warehouses: Vec<ParcelWarehouseReport>,
    #[schema(example = 1)]
    pub parcels_num: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ParcelWarehouseReport {
    #[schema(example = 1)]
    pub warehouse_id: usize,
    pub sizes: Vec<ParcelSizeReport>,
    #[schema(example = 1)]
    pub parcels_num: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ParcelSizeReport {
    #[schema(example = ParcelSize::L)]
    pub size: ParcelSize,
    pub parcels_id: Vec<usize>,
    #[schema(example = 1)]
    pub parcels_num: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct DeliveryReport {
    #[schema(example = "28-05-2023")]
    pub date: String,
    #[schema(example = 1)]
    pub parcels_num: usize,
    pub parcels: Vec<ParcelBody>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ParcelBody {
    #[schema(example = "Jan Kowalski")]
    pub recipient_name: String,
    #[schema(example = "jan.kowalski@example.net")]
    pub recipient_email: String,
    #[schema(example = "123456789")]
    pub recipient_phone: String,
    pub recipient_address: AddressBody,
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
