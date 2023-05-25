use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct AddBody {
    #[schema(example = "Jan")]
    pub firstname: String,
    #[schema(example = "Testowy")]
    pub lastname: String,
    #[schema(example = "jan@testowy.com")]
    pub email: String,
    #[schema(example = "123456789")]
    pub phone: String,
}
#[derive(Serialize, ToSchema)]
pub struct AddResponse {
    #[schema(example = 1)]
    pub id: usize,
}
