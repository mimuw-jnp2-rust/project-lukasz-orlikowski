use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

pub extern crate crypto;
pub extern crate jwt;
pub extern crate rustc_serialize;

use self::crypto::sha2::Sha256;
use self::jwt::{Header, Registered, Token};

pub struct ApiKey(pub String);

pub fn read_token(key: &str) -> Result<String, String> {
    let token =
        Token::<Header, Registered>::parse(key).map_err(|_| "Unable to parse key".to_string())?;
    if token.verify(b"secret_key", Sha256::new()) {
        token
            .claims
            .sub
            .ok_or_else(|| "Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Forward(()),
        }
    }
}
