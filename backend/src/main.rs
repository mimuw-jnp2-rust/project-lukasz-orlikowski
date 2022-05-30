use rocket::futures::future::join_all;
use rocket::http::{Status};
use types::{Credentials, TokenResponse };
use db::Connection;
use rocket::serde::json::Json;
use crate::auth::ApiKey;
use crate::types::{PrivateBoardData, TeamData, TeamBoardData, TeamBoardWithName};
use board::{PrivateBoard, TeamBoard};
use team::Team;
use list::List;

use self::auth::crypto::sha2::Sha256;
use self::auth::jwt::{
    Header,
    Registered,
    Token,
};
use user::User;
use task::Task;

pub mod types;
pub mod task;
pub mod team;
pub mod auth;
pub mod db;
pub mod user;
pub mod schema;
pub mod board;
pub mod list;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate rocket_sync_db_pools;

use rocket::{Rocket, Build, Response, Request};
use rocket::fairing::AdHoc;
use std::error::Error;
use std::str::FromStr;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, AllowedMethods, CorsOptions};

#[post("/login", data = "<credentials>")]
async fn login(credentials: Json<Credentials>, connection: Connection) ->  Result<Json<TokenResponse>, Status> {
    let header: Header = Default::default();
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();
    
    match User::by_username_and_password(username, password, &connection).await {
        None => {
            Err(Status::NotFound)
        },
        Some(user) => {
            let claims = Registered {
                sub: Some(user.username.into()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token.signed(b"secret_key", Sha256::new())
                .map(|message| Json(TokenResponse::new(true, message)))
                .map_err(|_| Status::InternalServerError)
        }
    }
}

#[post("/register", data = "<data>")]
async fn register(data: Json<User>, connection: Connection) -> Result<Json<bool>, Status> {
    let insert = User { id: None, ..data.into_inner() };
    match User::create(insert, &connection).await {
        Ok(_) => Ok(Json(true)),
        _ => Err(Status::NotFound)
    }
}

#[post("/private_board/create", data="<data>")]
async fn private_board(data: Json<PrivateBoardData>, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    match User::get_username_id(key.0, &connection).await {
        Some(user_id) => {
            let board = PrivateBoard{id: None, owner: user_id, name: data.name.clone()};
            match PrivateBoard::create(board, &connection).await {
                Ok(cnt) => Ok(Json(cnt > 0)),
                _ => Err(Status::NotFound)
            }
        },
        _ => Err(Status::NotFound) 
    }
}

#[post("/team_board/create", data="<data>")]
async fn team_board(data: Json<TeamBoardData>, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
        let board = TeamBoard{id: None, owner: data.owner, name: data.name.clone()};
        match TeamBoard::create(board, &connection).await {
            Ok(cnt) => Ok(Json(cnt > 0)),
            _ => Err(Status::NotFound)
        }
}

#[post("/team/create", data="<data>")]
async fn team_create(data: Json<TeamData>, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status>{
    let mut members: Vec<Option<i32>> = join_all(data.members.split(";").map(|x| async {User::get_username_id(x.to_string(), &connection).await})).await;
    let user_id = User::get_username_id(key.0, &connection).await;
    members.push(user_id);
    match Team::create(members, data.name.clone(), user_id.unwrap(),  &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound)
    }
}

#[get("/owned")]
async fn owned(connection: Connection, key: ApiKey) -> Result<Json<Vec<Team>>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    match Team::get_owned(user_id.unwrap(), &connection).await {
        Ok(teams) => Ok(Json(teams)),
        _ => Err(Status::NotFound)
    }
}

#[get("/private_board/get")]
async fn get_private_boards(connection: Connection, key: ApiKey) -> Result<Json<Vec<PrivateBoard>>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    println!("user_id: {:?}", user_id);
    match User::get_private_boards(user_id.unwrap(), &connection).await {
        Ok(boards) => Ok(Json(boards)),
        Err(sth) => {println!("{:?}", sth);Err(Status::NotFound)}
    }
}

#[get("/team_board/get")]
async fn get_team_boards(connection: Connection, key: ApiKey) -> Result<Json<Vec<TeamBoardWithName>>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    match Team::get_teams_boards(user_id.unwrap(), &connection).await {
        Ok(boards) => Ok(Json(boards)),
        _ => Err(Status::NotFound)
    }
}

#[post("/new_list", data="<data>")]
async fn new_list(data: Json<List>, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    let list = List { ..data.into_inner() };
    match List::create(list, user_id.unwrap(), &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound)
    }
}

#[get("/list/<board_type>/<id>")]
async fn get_list(board_type: String, id: i32, connection: Connection, key: ApiKey) -> Result<Json<Vec<List>>, Status> {
    match List::get(board_type, id, &connection).await {
        Ok(lists) => Ok(Json(lists)),
        _ => Err(Status::NotFound)
    }
}

#[get("/task/get/<id>")]
async fn get_tasks(id: i32, connection: Connection, key: ApiKey) -> Result<Json<Vec<Task>>, Status> {
    match Task::get(id, &connection).await {
        Ok(tasks) => Ok(Json(tasks)),
        _ => Err(Status::NotFound)
    }
}

#[get("/task/<id>")]
async fn get_task(id: i32, connection: Connection, key: ApiKey) -> Result<Json<Task>, Status> {
    match Task::get_single(id, &connection).await {
        Ok(task) => Ok(Json(task)),
        _ => Err(Status::NotFound)
    }
}

#[get("/private/delete/<id>")]
async fn delete_private(id: i32, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    match PrivateBoard::delete(id, &connection).await {
        Ok(_) => Ok(Json(true)),
        _ => Err(Status::NotFound)
    }
}

#[get("/team_board/delete/<id>")]
async fn delete_team_board(id: i32, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    match TeamBoard::delete(id, &connection).await {
        Ok(_) => Ok(Json(true)),
        _ => Err(Status::NotFound)
    }
}


#[get("/task/delete/<id>")]
async fn delete_task(id: i32, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    match Task::delete(id, &connection).await {
        Ok(_) => Ok(Json(true)),
        _ => Err(Status::NotFound)
    }
}


#[post("/task/create", data="<data>")]
async fn create_task(data: Json<Task>, connection: Connection, ket: ApiKey) -> Result<Json<bool>, Status> {
    let task = Task {..data.into_inner()};
    match Task::create(task, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound)
    }
}

#[post("/task/update", data="<data>")]
async fn update_task(data: Json<Task>, connection: Connection, ket: ApiKey) -> Result<Json<bool>, Status> {
    let task = Task {..data.into_inner()};
    match Task::update(task, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound)
    }
}
//#[get("/sensitive")]
//fn sensitive(key: ApiKey) -> String {
//    format!("Hello, you have been identified as {}", key.0)
//}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn =  Connection::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("can run migrations");

    rocket
}


#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true)
    .to_cors()?;

    rocket::build()
        .mount("/", routes![update_task, get_tasks, delete_task, delete_team_board, login, register, private_board, team_create, team_board, owned, get_private_boards, get_team_boards, delete_private, new_list, get_list, create_task, get_task])
        .attach(cors).attach(Connection::fairing()).attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .launch()
        .await?;

    Ok(())
}
