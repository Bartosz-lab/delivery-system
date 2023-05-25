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
    pub user_id: usize,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
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
                        let user: ClaimsData = serde_json::from_str(c.claims.sub.as_str()).unwrap();
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
                        "You are not logged in, please provide Token",
                    )))
                }
            }
        }
    }
}
