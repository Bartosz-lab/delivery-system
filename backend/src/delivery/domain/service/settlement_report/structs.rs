use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::delivery::domain::value_objects::ParcelSize;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SettlementTotalReport {
    #[schema(example = "28-05-2023")]
    pub start_date: String,
    #[schema(example = "28-05-2023")]
    pub end_date: String,
    pub warehouses: Vec<SettlementWarehouseReport>,
    pub price: MoneyBody,
    #[schema(example = 1)]
    pub parcels_num: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SettlementWarehouseReport {
    #[schema(example = 1)]
    pub warehouse_id: i32,
    pub sizes: Vec<SettlementSizeReport>,
    pub price: MoneyBody,
    #[schema(example = 1)]
    pub parcels_num: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SettlementSizeReport {
    #[schema(example = ParcelSize::L)]
    pub size: ParcelSize,
    pub unit_price: MoneyBody,
    #[schema(example = json![vec![1]])]
    pub parcels_id: Vec<i32>,
    pub price: MoneyBody,
    #[schema(example = 1)]
    pub parcels_num: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct MoneyBody {
    #[schema(example = "100.00")]
    pub price: String,
    #[schema(example = "PLN")]
    pub currency: String,
}
