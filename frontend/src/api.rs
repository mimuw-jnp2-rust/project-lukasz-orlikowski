use gloo_net::http::Request;
use gloo_net::Error;

use crate::types::{
    BoardUpdate, List, Login, LoginResponse, PrivateBoard, PrivateBoardData, Task, Team, TeamBoard,
    TeamBoardData, TeamData,
};

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

    let res = request.send().await?.json().await?;
    Ok(res)
}

pub async fn register(username: &str, password: &str) -> Result<bool, Error> {
    let request = get_login_register_req("register", username, password)?;
    let res = request.send().await?.json().await?;
    Ok(res)
}

pub async fn create_private_board(name: &str, token: &str) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "private_board/create");
    let body_obj = PrivateBoardData {
        name: name.to_owned(),
    };
    let res = Request::post(url.as_str())
        .header("Authorization", token)
        .json(&body_obj)?
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

pub async fn create_team_board(name: &str, team: i32, token: &str) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "team_board/create");
    let body_obj = TeamBoardData {
        name: name.to_owned(),
        owner: team,
    };
    Request::post(url.as_str())
        .header("Authorization", token)
        .json(&body_obj)?
        .send()
        .await?
        .json()
        .await
}

pub async fn create_team(name: &str, members: &str, token: &str) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "team/create");
    let body_obj = TeamData {
        name: name.to_owned(),
        members: members.to_owned(),
    };
    let res = Request::post(url.as_str())
        .header("Authorization", token)
        .json(&body_obj)?
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

pub async fn get_user_teams(token: &str) -> Result<Vec<Team>, Error> {
    let url = format!("{}{}", BACKEND, "owned");
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn get_private_boards(token: &str) -> Result<Vec<PrivateBoard>, Error> {
    let url = format!("{}{}", BACKEND, "private_board/get");
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn get_team_boards(token: &str) -> Result<Vec<TeamBoard>, Error> {
    let url = format!("{}{}", BACKEND, "team_board/get");
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn create_list(token: &str, list: List) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "new_list");
    Request::post(url.as_str())
        .header("Authorization", token)
        .json(&list)?
        .send()
        .await?
        .json()
        .await
}

pub async fn update_board(
    token: &str,
    id: i32,
    name: String,
    board_type: &str,
) -> Result<bool, Error> {
    let url = format!("{}{}/update/{}", BACKEND, board_type, id);
    let board = BoardUpdate { name };
    Request::post(url.as_str())
        .header("Authorization", token)
        .json(&board)?
        .send()
        .await?
        .json()
        .await
}

pub async fn delete_list(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}list_delete/{}", BACKEND, id);
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn create_task(token: &str, task: Task) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "task/create");
    Request::post(url.as_str())
        .header("Authorization", token)
        .json(&task)?
        .send()
        .await?
        .json()
        .await
}

pub async fn update_task(token: &str, task: Task) -> Result<bool, Error> {
    let url = format!("{}{}", BACKEND, "task/update");
    Request::post(url.as_str())
        .header("Authorization", token)
        .json(&task)?
        .send()
        .await?
        .json()
        .await
}

pub async fn get_lists(board_id: i32, board_type: String, token: &str) -> Result<Vec<List>, Error> {
    let url = format!("{}{}/{}/{}", BACKEND, "list", board_type, board_id);
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn get_tasks(token: &str, list_id: i32) -> Result<Vec<Task>, Error> {
    let url = format!("{}{}{}", BACKEND, "task/get/", list_id);
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn get_task(token: &str, id: i32) -> Result<Task, Error> {
    let url = format!("{}{}{}", BACKEND, "task/", id);
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn delete_private(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}{}{}", BACKEND, "private/delete/", id);
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn delete_team_board(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}{}{}", BACKEND, "team_board/delete/", id);
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}

pub async fn delete_task(token: &str, id: i32) -> Result<bool, Error> {
    let url = format!("{}{}{}", BACKEND, "task/delete/", id);
    Request::get(url.as_str())
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await
}
