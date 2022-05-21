use serde::{Deserialize, Serialize};
use yew::Properties;

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

#[derive(Serialize, Deserialize, Debug, Properties, PartialEq, Clone)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub owner: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamBoardData {
    pub name: String,
    pub owner: i32
}


