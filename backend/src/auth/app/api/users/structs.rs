use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::auth::domain::Role;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserBody {
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
#[derive(Serialize, ToSchema)]
pub struct AddResponse {
    #[schema(example = 1)]
    pub id: usize,
}

#[derive(Serialize, ToSchema)]
pub struct RolesResponse {
    #[schema(example = json![vec![Role::Admin,  Role::PartnerApi(1)]])]
    pub roles: Vec<Role>,
}
