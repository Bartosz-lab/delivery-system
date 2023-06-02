use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::delivery::app::api::structs::AddressBody;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "TradePartner")]
pub struct TradePartnerBody {
    #[schema(example = "Y-Kom sp. z o.o.")]
    #[schema(default = "_")]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "TradePartnerAdmin")]
pub struct TradePartnerAdminBody {
    #[schema(example = 1)]
    pub id: usize,
    #[schema(example = "Y-Kom sp. z o.o.")]
    #[schema(default = "_")]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "Money")]
pub struct MoneyBody {
    #[schema(example = "100.00")]
    pub price: String,
    #[schema(example = "PLN")]
    pub currency: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "Warehouse")]
pub struct WarehouseBody {
    #[schema(example = 1)]
    pub id: Option<usize>,
    #[schema(example = "Magazyn 1")]
    #[schema(default = "_")]
    pub name: Option<String>,
    pub address: Option<AddressBody>,
}
