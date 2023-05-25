use serde::Deserialize;
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
