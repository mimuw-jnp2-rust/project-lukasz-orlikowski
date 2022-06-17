use rocket::serde::{Deserialize, Serialize};

use crate::board::TeamBoard;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    success: bool,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TeamBoardWithName {
    pub id: Option<i32>,
    pub name: String,
    pub owner: i32,
    pub team_name: String,
}

impl TeamBoardWithName {
    pub fn new(boards: Vec<TeamBoard>, name: String) -> Vec<TeamBoardWithName> {
        boards
            .into_iter()
            .map(|x| TeamBoardWithName {
                id: x.id,
                name: x.name,
                owner: x.owner,
                team_name: name.clone(),
            })
            .collect()
    }
}

impl TokenResponse {
    pub fn new(success: bool, token: String) -> TokenResponse {
        TokenResponse { success, token }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PrivateBoardData {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TeamBoardData {
    pub name: String,
    pub owner: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TeamData {
    pub name: String,
    pub members: String,
}

#[derive(Serialize, Deserialize)]
pub struct BoardUpdate {
    pub name: String,
}


#[derive(Deserialize)]
pub struct TimerData {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TaskFilter {
    pub name: String,
    pub place: String,
    pub members: String,
    pub deadline_start: String,
    pub deadline_end: String,
    pub points_min: Option<i32>,
    pub points_max: Option<i32>,
    pub tags: String
}