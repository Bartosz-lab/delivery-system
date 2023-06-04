use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginBody {
    #[schema(example = "jan@testowy.com")]
    pub login: String,
    #[schema(example = "Pass1234")]
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ChangePassBody {
    #[schema(example = "Pass1234")]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct JWT {
    pub token: String,
}
