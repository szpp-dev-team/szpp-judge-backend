use actix_web::{error::ErrorUnauthorized, http::header, Error, FromRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub id: i32,
    pub role: String,
}

use std::env;
pub static SECRET: Lazy<String> = Lazy::new(|| env::var("JWT_SECRET").unwrap());

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Claims, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth = req.headers().get(header::AUTHORIZATION);
        match auth {
            Some(auth) => {
                let token = auth
                    .to_str()
                    .unwrap()
                    .split("Bearer")
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .trim();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(SECRET.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(c) => ok(c.claims),
                    Err(_e) => err(ErrorUnauthorized("invalid jwt token")),
                }
            }
            None => err(ErrorUnauthorized("blocked")),
        }
    }
}
