

use serde::{Deserialize, Serialize};
use wasm_timer::{SystemTime, UNIX_EPOCH};
use yew::Properties;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LoginResponse {
    success: bool,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct PrivateBoardData {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct TeamData {
    pub name: String,
    pub members: String,
}

#[derive(Serialize, Deserialize, Debug, Properties, PartialEq, Clone)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub owner: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamBoardData {
    pub name: String,
    pub owner: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct TeamBoard {
    pub id: Option<i32>,
    pub name: String,
    pub owner: i32,
    pub team_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Properties)]
pub struct PrivateBoard {
    pub id: Option<i32>,
    pub name: String,
    pub owner: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Properties)]
pub struct BoardUpdate {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Properties)]
pub struct List {
    pub id: Option<i32>,
    pub name: String,
    pub board: i32,
    pub board_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: Option<i32>,
    pub name: String,
    pub list: i32,
    pub note: Option<String>,
    pub place: Option<String>,
    pub members: Option<String>,
    pub deadline: String,
    pub subtasks: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timer {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub status: String,
    pub time: u64,
    pub start: Option<u64>
}

impl Timer {
    pub fn get_time(&self) -> u64 {
        if self.status == "active" {
            let time = SystemTime::now();
            let since_the_epoch = time
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            since_the_epoch.as_secs() + self.time  - self.start.unwrap() as u64
        }
        else {
            self.time
        }
    }
}

#[derive(Serialize)]
pub struct TimerData {
    pub name: String
}
