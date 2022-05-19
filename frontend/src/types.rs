use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LoginResponse {
    success: bool,
    pub token: String
}

#[derive(Deserialize, Serialize)]
pub struct  Login {
    pub username: String,
    pub password: String
}

