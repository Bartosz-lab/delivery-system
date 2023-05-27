use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::delivery::domain::{value_objects::ParcelSize, Address};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TradePartnerBody {
    #[schema(example = "Y-Kom sp. z o.o.")]
    #[schema(default = "_")]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TradePartnerAdminBody {
    #[schema(example = 1)]
    pub id: usize,
    #[schema(example = "Y-Kom sp. z o.o.")]
    #[schema(default = "_")]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TradePartnerListBody {
    pub list: Vec<TradePartnerAdminBody>,
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WarehouseAdminBody {
    #[schema(example = 1)]
    pub id: usize,
    #[schema(example = "Magazyn 1")]
    pub name: String,
    #[schema(example = 1)]
    pub trade_partner_id: usize,
    pub address: Address,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WarehouseBody {
    #[schema(example = "Magazyn 1")]
    #[schema(default = "_")]
    pub name: String,
    pub address: AddressBody,
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
