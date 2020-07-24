use std::env;

use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::users::time::get_expire_time;

#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub sub: String,
    pub name: String,
    pub exp: u128,
}

pub fn encode(name: &str) -> Option<String> {
    let auth_payload = AuthPayload {
        sub: "token".to_owned(),
        name: name.to_owned(),
        exp: get_expire_time(),
    };

    let auth_token = match jwt_encode(
        &Header::default(),
        &auth_payload,
        &EncodingKey::from_secret(&env::var("jwt_secret").unwrap().as_ref()),
    ) {
        Ok(value) => value,
        Err(_) => return None,
    };

    Some(auth_token)
}

pub fn decode(token: &str) -> Option<AuthPayload> {
    let validation = Validation {
        sub: Some("token".to_owned()),
        ..Validation::default()
    };

    let auth_token = match jwt_decode::<AuthPayload>(
        &token,
        &DecodingKey::from_secret(&env::var("jwt_secret").unwrap().as_ref()),
        &validation,
    ) {
        Ok(value) => value,
        Err(_) => return None,
    };

    Some(auth_token.claims)
}
