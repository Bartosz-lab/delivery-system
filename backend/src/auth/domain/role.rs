use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Courier,
    PartnerUser(String),
    PartnerApi(String),
}
