use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::delivery::domain::value_objects::ParcelSize;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TradePartnerBody {
    #[schema(example = "Y-Kom sp. z o.o.")]
    #[schema(default = "_")]
    pub name: String,
}
#[derive(Serialize, ToSchema)]
pub struct AddResponse {
    #[schema(example = 1)]
    pub id: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PriceListBody {
    #[schema(example = json![vec![(ParcelSize::L, "100.00")]])]
    pub list: Vec<(ParcelSize, MoneyBody)>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MoneyBody {
    #[schema(example = "100.00")]
    pub price: String,
    #[schema(example = "PLN")]
    pub currency: String,
}
