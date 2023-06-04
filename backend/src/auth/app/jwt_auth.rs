use actix_web::error::{ErrorForbidden, ErrorUnauthorized};
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::auth::domain::Role;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimsData {
    pub user_id: i32,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_info: ClaimsData,
    pub iat: usize,
    pub exp: usize,
}

pub struct AuthExtractor {
    pub user: ClaimsData,
}

impl FromRequest for AuthExtractor {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        match token {
            None => ready(Err(ErrorUnauthorized(
                "You are not logged in, please provide Token",
            ))),
            Some(token_str) => {
                let decode = decode::<TokenClaims>(
                    token_str.as_str(),
                    &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
                    &Validation::default(),
                );

                match decode {
                    Ok(c) => {
                        let user: ClaimsData = c.claims.user_info;
                        req.extensions_mut().insert::<ClaimsData>(user.clone());

                        ready(Ok(AuthExtractor { user }))
                    }
                    Err(e) => ready(Err(ErrorUnauthorized(format!(
                        "Invalid Token {}",
                        e.to_string()
                    )))),
                }
            }
        }
    }
}

pub struct AdminExtractor;

impl FromRequest for AdminExtractor {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let binding = req.extensions();
        let user_opt = binding.get::<ClaimsData>();

        match user_opt {
            None => ready(Err(ErrorUnauthorized(
                "You are not logged in, please provide Token",
            ))),
            Some(user) => {
                if user.roles.contains(&Role::Admin) {
                    ready(Ok(AdminExtractor))
                } else {
                    ready(Err(ErrorForbidden(
                        "You are not have access to this endpoint",
                    )))
                }
            }
        }
    }
}

pub struct CourierExtractor;

impl FromRequest for CourierExtractor {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<ClaimsData>() {
            None => ready(Err(ErrorUnauthorized(
                "You are not logged in, please provide Token",
            ))),
            Some(user) => {
                if user.roles.iter().any(|role| match role {
                    Role::Admin | Role::Courier => true,
                    _ => false,
                }) {
                    ready(Ok(CourierExtractor))
                } else {
                    ready(Err(ErrorForbidden(
                        "You are not have access to this endpoint",
                    )))
                }
            }
        }
    }
}

pub struct TradePartnerExtractor {
    pub trade_partner_id: i32,
}

impl FromRequest for TradePartnerExtractor {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let binding = req.extensions();
        let user_opt = binding.get::<ClaimsData>();

        match user_opt {
            None => ready(Err(ErrorUnauthorized(
                "You are not logged in, please provide Token",
            ))),
            Some(user) => {
                let role_opt = user
                    .roles
                    .iter()
                    .filter(|role| match role {
                        Role::PartnerUser(_) => true,
                        _ => false,
                    })
                    .next();
                match role_opt {
                    Some(role) => match role {
                        Role::PartnerUser(id) => ready(Ok(TradePartnerExtractor {
                            trade_partner_id: *id,
                        })),
                        _ => ready(Err(ErrorForbidden(
                            "You are not have access to this endpoint",
                        ))),
                    },

                    None => ready(Err(ErrorForbidden(
                        "You are not have access to this endpoint",
                    ))),
                }
            }
        }
    }
}
pub struct TradePartnerApiExtractor {
    pub trade_partner_id: i32,
}

impl FromRequest for TradePartnerApiExtractor {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let binding = req.extensions();
        let user_opt = binding.get::<ClaimsData>();

        match user_opt {
            None => ready(Err(ErrorUnauthorized(
                "You are not logged in, please provide Token",
            ))),
            Some(user) => {
                let role_opt = user
                    .roles
                    .iter()
                    .filter(|role| match role {
                        Role::PartnerUser(_) | Role::PartnerApi(_) => true,
                        _ => false,
                    })
                    .next();
                match role_opt {
                    Some(role) => match role {
                        Role::PartnerUser(id) | Role::PartnerApi(id) => {
                            ready(Ok(TradePartnerApiExtractor {
                                trade_partner_id: *id,
                            }))
                        }
                        _ => ready(Err(ErrorForbidden(
                            "You are not have access to this endpoint",
                        ))),
                    },

                    None => ready(Err(ErrorForbidden(
                        "You are not have access to this endpoint",
                    ))),
                }
            }
        }
    }
}
