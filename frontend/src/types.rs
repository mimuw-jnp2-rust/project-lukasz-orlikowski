use serde::{Deserialize, Serialize};
use wasm_timer::{SystemTime, UNIX_EPOCH};
use yew::Properties;

use crate::utils::set_value;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LoginResponse {
    success: bool,
    pub token: String,
}

#[derive(Serialize, Clone, PartialEq, Deserialize, Debug)]
pub struct MilestoneCreate {
    pub id: Option<i32>,
    pub name: String,
    pub board_id: i32,
    pub board_type: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Milestone {
    pub id: Option<i32>,
    pub name: String,
    pub done: i32,
    pub total: i32,
    pub board_id: i32,
    pub board_type: String,
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
    pub points: i32,
    pub tags: String,
    pub done: i32,
    pub milestone: Option<i32>,
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
    pub tags: String,
}

impl TaskFilter {
    pub fn set_filters(&self) {
        set_value("nameTaskFilter", self.name.clone().as_str());
        set_value("placeFilter", self.place.clone().as_str());
        set_value("membersFilter", self.members.clone().as_str());
        set_value("deadlineStart", self.deadline_start.clone().as_str());
        set_value("deadlineEnd", self.deadline_end.clone().as_str());
        if self.points_min.is_some() {
            set_value("pointsMin", self.points_min.unwrap().to_string().as_str());
        } else {
            set_value("pointsMin", "");
        }
        if self.points_max.is_some() {
            set_value("pointsMin", self.points_max.unwrap().to_string().as_str());
        } else {
            set_value("pointsMin", "");
        }
        set_value("tagsFilter", self.tags.clone().as_str());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timer {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub status: String,
    pub time: u64,
    pub start: Option<u64>,
}

impl Timer {
    pub fn get_time(&self) -> u64 {
        if self.status == "active" {
            let time = SystemTime::now();
            let since_the_epoch = time
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            since_the_epoch.as_secs() + self.time - self.start.unwrap() as u64
        } else {
            self.time
        }
    }
}

#[derive(Serialize)]
pub struct TimerData {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Properties)]
pub struct IdProp {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Properties)]
pub struct BoardProp {
    pub id: i32,
    pub board_type: String,
    pub milestones: Option<Vec<Milestone>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub id: Option<i32>,
    pub name: String,
    pub list: i32,
    pub note: Option<String>,
    pub place: Option<String>,
    pub members: Option<String>,
    pub timestamp: String,
    pub action: String,
    pub task_id: i32,
    pub deadline: String,
    pub subtasks: String,
    pub points: i32,
    pub tags: String,
}
