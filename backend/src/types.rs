use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
   pub username: String,
   pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
   success: bool,
   token: String
}

impl TokenResponse {
    pub fn new(success: bool, token: String) -> TokenResponse {
       TokenResponse {success, token}
    }
}

#[derive(Serialize, Deserialize)]
pub struct PrivateBoardData {
   pub name: String,
}