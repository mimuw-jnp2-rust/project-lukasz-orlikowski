use gloo_net::http::Request;
use gloo_net::Error;
use wasm_bindgen::JsValue;

use crate::types::{LoginResponse, Login, PrivateBoardData};

static BACKEND: &str = "http://localhost:8000/";


fn get_login_register_req(url: &str, username: &str, password: &str) -> Result<Request, Error> {
    let url = format!("{}{}", BACKEND, url);
    let body_obj = Login{username: username.to_owned(), password: password.to_owned()};
    Ok(Request::post(url.as_str()).json(&body_obj)?)
}

pub async fn login(username: &str, password: &str) -> Result<LoginResponse, Error>  {

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
    let body_obj = PrivateBoardData{name: name.to_owned()};
    let res = Request::post(url.as_str()).header("Authorization", &token).json(&body_obj)?.send().await?.json().await?;
    Ok(res)
}