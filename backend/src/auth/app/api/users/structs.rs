use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddUser {
    #[schema(example = "Jan")]
    #[schema(default = "_")]
    pub firstname: String,
    #[schema(example = "Testowy")]
    #[schema(default = "_")]
    pub lastname: String,
    #[schema(example = "jan@testowy.com")]
    #[schema(default = "_")]
    pub email: String,
    #[schema(example = "123456789")]
    #[schema(default = "_")]
    pub phone: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "User")]
pub struct UserBody {
    #[schema(example = "Jan")]
    #[schema(default = "_")]
    pub firstname: Option<String>,
    #[schema(example = "Testowy")]
    #[schema(default = "_")]
    pub lastname: Option<String>,
    #[schema(example = "jan@testowy.com")]
    #[schema(default = "_")]
    pub email: Option<String>,
    #[schema(example = "123456789")]
    #[schema(default = "_")]
    pub phone: Option<String>,
}
