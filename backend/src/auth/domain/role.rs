use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum Role {
    Admin,
    Courier,
    PartnerUser(i32),
    PartnerApi(i32),
}
