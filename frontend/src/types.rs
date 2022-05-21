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

#[derive(Deserialize, Serialize)]
pub struct PrivateBoardData {
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct TeamData {
    pub name: String,
    pub members: String
}

