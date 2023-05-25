use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct AuthExtractor {
    pub user_id: usize,
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
                        let user_id = c.claims.sub.parse::<usize>().unwrap();
                        req.extensions_mut().insert::<usize>(user_id);

                        ready(Ok(AuthExtractor { user_id }))
                    }
                    Err(_) => ready(Err(ErrorUnauthorized("Invalid Token"))),
                }
            }
        }
    }
}
