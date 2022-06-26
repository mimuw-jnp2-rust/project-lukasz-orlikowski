use gloo_net::http::{Request, Response};
use gloo_net::Error;
use serde::Serialize;

use crate::{types::{
    BoardUpdate, List, Login, LoginResponse, PrivateBoard, PrivateBoardData, Task, Team, TeamBoard,
    TeamBoardData, TeamData, Timer, TimerData, TaskFilter, Log, Milestone, MilestoneCreate,
}};

static BACKEND: &str = "http://localhost:8000/";

fn get_login_register_req(url: &str, username: &str, password: &str) -> Result<Request, Error> {
    let url = format!("{}{}", BACKEND, url);
    let body_obj = Login {
        username: username.to_owned(),
        password: password.to_owned(),
    };
    Request::post(url.as_str()).json(&body_obj)
}

pub async fn login(username: &str, password: &str) -> Result<LoginResponse, Error> {
    let request = get_login_register_req("login", username, password)?;

    request.send().await?.json().await
}

pub async fn register(username: &str, password: &str) -> Result<bool, Error> {
    let request = get_login_register_req("register", username, password)?;
    request.send().await?.json().await
}

async fn send_request<T> (url: String, body_obj: T, token: &str) -> Result<Response, Error> where T: Serialize{
    Request::post(url.as_str())
        .header("Authorization", token)
        .json(&body_obj)?
        .send()
        .await
}

async fn get(url: String, token: &str) -> Result<Response, Error> {
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await
}

pub async fn create_private_board(name: &str, token: &str) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "private_board/create");
    let body_obj = PrivateBoardData {
        name: name.to_owned(),
    };
    send_request(url, body_obj, token).await?.json()
    .await
}

pub async fn create_team_board(name: &str, team: i32, token: &str) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "team_board/create");
    let body_obj = TeamBoardData {
        name: name.to_owned(),
        owner: team,
    };
    send_request(url, body_obj, token).await?.json()
    .await
}

pub async fn create_team(name: &str, members: &str, token: &str) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "team/create");
    let body_obj = TeamData {
        name: name.to_owned(),
        members: members.to_owned(),
    };
    send_request(url, body_obj, token).await?.json().await
}

pub async fn get_user_teams(token: &str) -> Result<Vec<Team>, Error> {
    let url = format!("{}{}", BACKEND, "owned");
    get(url, token).await?.json().await
}

pub async fn get_private_boards(token: &str) -> Result<Vec<PrivateBoard>, Error> {
    let url = format!("{}{}", BACKEND, "private_board/get");
    get(url, token)
        .await?
        .json()
        .await
}

pub async fn get_team_boards(token: &str) -> Result<Vec<TeamBoard>, Error> {
    let url = format!("{}{}", BACKEND, "team_board/get");
    get(url, token).await?.json().await
}

pub async fn create_list(token: &str, list: List) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "new_list");
    send_request(url, list, token).await?.json().await
}

pub async fn update_board(
    token: &str,
    id: i32,
    name: String,
    board_type: &str,
) -> Result<bool, Error> {
    let url = format!("{}{}/update/{}", BACKEND, board_type, id);
    let board = BoardUpdate { name };
    send_request(url, board, token).await?.json().await
}

pub async fn delete_list(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}list_delete/{}", BACKEND, id);
    get(url, token).await?.json().await
}

pub async fn create_task(token: &str, task: Task) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "task/create");
    send_request(url, task, token).await?.json().await
}

pub async fn update_task(token: &str, task: Task) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "task/update");
    send_request(url, task, token).await?.json().await
}

pub async fn get_lists(board_id: i32, board_type: String, token: &str) -> Result<Vec<List>, Error> {
    let url = format!("{}{}/{}/{}", BACKEND, "list", board_type, board_id);
    get(url, token).await?.json().await
}

pub async fn get_logs(task_id: i32, token: &str) -> Result<Vec<Log>, Error> {
    let url = format!("{}logs/get/{}", BACKEND, task_id);
    get(url, token).await?.json().await
}


pub async fn get_tasks(token: &str, list_id: i32, filter: Option<TaskFilter>) -> Result<Vec<Task>, Error> {
    let url = format!("{}{}{}", BACKEND, "task/get/", list_id);
    if filter.is_none() {
        get(url, token).await?.json().await
    }
    else {
        send_request(url, filter.unwrap(), token)
        .await?
        .json()
        .await
    }
}

pub async fn get_task(token: &str, id: i32) -> Result<Task, Error> {
    let url = format!("{}{}{}", BACKEND, "task/", id);
    get(url, token).await?.json().await
}

pub async fn delete_private(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}{}{}", BACKEND, "private/delete/", id);
    get(url, token).await?.json().await
}

pub async fn delete_team_board(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}{}{}", BACKEND, "team_board/delete/", id);
    get(url, token).await?.json().await
}

pub async fn delete_task(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}{}{}", BACKEND, "task/delete/", id);
    get(url, token).await?.json().await
}

pub async fn get_timers(token: &str) -> Result<Vec<Timer>, Error> {
    let url = format!("{}timers/get", BACKEND);
    get(url, token).await?.json().await
}

pub async fn update_timer(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}timer/update/{}", BACKEND, id);
    get(url, token).await?.json().await
}

pub async fn delete_timer(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}timer/delete/{}", BACKEND, id);
    get(url, token).await?.json().await
}

pub async fn create_timer(token: &str, name: &str) -> Result<bool, Error> {
    let timer = TimerData {name: name.to_owned()};
    let url = format!("{}{}", BACKEND, "timer/create");
    send_request(url, timer, token).await?.json().await
}

pub async fn get_milestones(id: i32, board_type: String, token: &str) -> Result<Vec<Milestone>, Error> {
    let url = format!("{}milestone/get/{}/{}", BACKEND, id, board_type);
    get(url, token).await?.json().await
}

pub async fn create_milestone(token: &str, milestone: MilestoneCreate) -> Result<bool, Error> {
    let url = format!("{}milestone/create", BACKEND);
    send_request(url, milestone, token).await?.json().await
}
