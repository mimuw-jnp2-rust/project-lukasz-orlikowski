use crate::auth::ApiKey;
use crate::types::{BoardUpdate, PrivateBoardData, TeamBoardData, TeamBoardWithName, TeamData};
use board::{PrivateBoard, TeamBoard};
use db::Connection;
use list::List;
use log::Log;
use milestone::{MilestoneResponse, Milestone};
use rocket::futures::future::join_all;
use rocket::http::Status;
use rocket::serde::json::Json;
use team::Team;
use timer::Timer;
use types::{Credentials, TokenResponse, TimerData, TaskFilter};
use utils::get_time;

use self::auth::crypto::sha2::Sha256;
use self::auth::jwt::{Header, Registered, Token};
use task::Task;
use user::User;

pub mod utils;
pub mod milestone;
pub mod auth;
pub mod board;
pub mod db;
pub mod list;
pub mod schema;
pub mod task;
pub mod team;
pub mod types;
pub mod user;
pub mod timer;
pub mod log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket_sync_db_pools;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use std::error::Error;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[post("/login", data = "<credentials>")]
async fn login(
    credentials: Json<Credentials>,
    connection: Connection,
) -> Result<Json<TokenResponse>, Status> {
    let header: Header = Default::default();
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();

    match User::by_username_and_password(username, password, &connection).await {
        None => Err(Status::NotFound),
        Some(user) => {
            let claims = Registered {
                sub: Some(user.username),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token
                .signed(b"secret_key", Sha256::new())
                .map(|message| Json(TokenResponse::new(true, message)))
                .map_err(|_| Status::InternalServerError)
        }
    }
}

#[post("/register", data = "<data>")]
async fn register(data: Json<User>, connection: Connection) -> Result<Json<bool>, Status> {
    let insert = User {
        id: None,
        ..data.into_inner()
    };
    match User::create(insert, &connection).await {
        Ok(_) => Ok(Json(true)),
        _ => Err(Status::NotFound),
    }
}

#[post("/private_board/create", data = "<data>")]
async fn private_board(
    data: Json<PrivateBoardData>,
    connection: Connection,
    key: ApiKey,
) -> Result<Json<bool>, Status> {
    match User::get_username_id(key.0, &connection).await {
        Some(user_id) => {
            let board = PrivateBoard {
                id: None,
                owner: user_id,
                name: data.name.clone(),
            };
            match PrivateBoard::create(board, &connection).await {
                Ok(cnt) => Ok(Json(cnt > 0)),
                _ => Err(Status::NotFound),
            }
        }
        _ => Err(Status::NotFound),
    }
}

#[post("/team_board/create", data = "<data>")]
async fn team_board(
    data: Json<TeamBoardData>,
    connection: Connection,
    key: ApiKey,
) -> Result<Json<bool>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    if Team::has_access(data.owner, user_id.unwrap(), &connection)
        .await
        .is_err()
    {
        return Err(Status::NotFound);
    }
    let board = TeamBoard {
        id: None,
        owner: data.owner,
        name: data.name.clone(),
    };
    match TeamBoard::create(board, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound),
    }
}

#[post("/team/create", data = "<data>")]
async fn team_create(
    data: Json<TeamData>,
    connection: Connection,
    key: ApiKey,
) -> Result<Json<bool>, Status> {
    let mut members: Vec<Option<i32>> = join_all(
        data.members
            .split(';')
            .map(|x| async { User::get_username_id(x.to_string(), &connection).await }),
    )
    .await;
    let user_id = User::get_username_id(key.0, &connection).await;
    members.push(user_id);
    match Team::create(members, data.name.clone(), user_id.unwrap(), &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound),
    }
}

#[get("/owned")]
async fn owned(connection: Connection, key: ApiKey) -> Result<Json<Vec<Team>>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    match Team::get_owned(user_id.unwrap(), &connection).await {
        Ok(teams) => Ok(Json(teams)),
        _ => Err(Status::NotFound),
    }
}

#[get("/private_board/get")]
async fn get_private_boards(
    connection: Connection,
    key: ApiKey,
) -> Result<Json<Vec<PrivateBoard>>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    match User::get_private_boards(user_id.unwrap(), &connection).await {
        Ok(boards) => Ok(Json(boards)),
        Err(sth) => {
            println!("{:?}", sth);
            Err(Status::NotFound)
        }
    }
}

#[get("/team_board/get")]
async fn get_team_boards(
    connection: Connection,
    key: ApiKey,
) -> Result<Json<Vec<TeamBoardWithName>>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    match Team::get_teams_boards(user_id.unwrap(), &connection).await {
        Ok(boards) => Ok(Json(boards)),
        _ => Err(Status::NotFound),
    }
}

#[get("/logs/get/<id>")]
async fn get_logs(
    id: i32,
    connection: Connection,
    key: ApiKey,
) -> Result<Json<Vec<Log>>, Status> {
    match Log::get(id, &connection).await {
        Ok(logs) => Ok(Json(logs)),
        _ => Err(Status::NotFound),
    }
}

#[post("/new_list", data = "<data>")]
async fn new_list(
    data: Json<List>,
    connection: Connection,
    _key: ApiKey,
) -> Result<Json<bool>, Status> {
    let list = List {
        ..data.into_inner()
    };
    match List::create(list, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound),
    }
}

#[get("/list/<board_type>/<id>")]
async fn get_list(
    board_type: String,
    id: i32,
    connection: Connection,
    _key: ApiKey,
) -> Result<Json<Vec<List>>, Status> {
    match List::get(board_type, id, &connection).await {
        Ok(lists) => Ok(Json(lists)),
        _ => Err(Status::NotFound),
    }
}

#[get("/task/get/<id>")]
async fn get_tasks(
    id: i32,
    connection: Connection,
    _key: ApiKey,
) -> Result<Json<Vec<Task>>, Status> {
    match Task::get(id, &connection).await {
        Ok(tasks) => Ok(Json(tasks)),
        _ => Err(Status::NotFound),
    }
}

#[post("/task/get/<id>", data="<data>")]
async fn filter_tasks(
    id: i32,
    data: Json<TaskFilter>,
    connection: Connection,
    _key: ApiKey,
) -> Result<Json<Vec<Task>>, Status> {
    match Task::filter(id, data.into_inner(), &connection).await {
        Ok(tasks) => Ok(Json(tasks)),
        _ => Err(Status::NotFound),
    }
}

#[get("/task/<id>")]
async fn get_task(id: i32, connection: Connection, _key: ApiKey) -> Result<Json<Task>, Status> {
    match Task::get_single(id, &connection).await {
        Ok(task) => Ok(Json(task)),
        _ => Err(Status::NotFound),
    }
}

#[get("/private/delete/<id>")]
async fn delete_private(
    id: i32,
    connection: Connection,
    _key: ApiKey,
) -> Result<Json<bool>, Status> {
    match PrivateBoard::delete(id, &connection).await {
        Ok(_) => Ok(Json(true)),
        _ => Err(Status::NotFound),
    }
}

#[get("/team_board/delete/<id>")]
async fn delete_team_board(
    id: i32,
    connection: Connection,
    _key: ApiKey,
) -> Result<Json<bool>, Status> {
    match TeamBoard::delete(id, &connection).await {
        Ok(_) => Ok(Json(true)),
        _ => Err(Status::NotFound),
    }
}

#[get("/task/delete/<id>")]
async fn delete_task(id: i32, connection: Connection, _key: ApiKey) -> Result<Json<bool>, Status> {
    match Task::delete(id, &connection).await {
        Ok(_) => Ok(Json(true)),
        _ => Err(Status::NotFound),
    }
}

#[post("/task/create", data = "<data>")]
async fn create_task(
    data: Json<Task>,
    connection: Connection,
    _ket: ApiKey,
) -> Result<Json<bool>, Status> {
    let task = Task {
        ..data.into_inner()
    };
    match Task::create(task, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound),
    }
}

#[post("/task/update", data = "<data>")]
async fn update_task(
    data: Json<Task>,
    connection: Connection,
    _ket: ApiKey,
) -> Result<Json<bool>, Status> {
    let task = Task {
        ..data.into_inner()
    };
    match Task::update(task, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound),
    }
}

#[post("/private/update/<id>", data = "<data>")]
async fn update_private(
    data: Json<BoardUpdate>,
    id: i32,
    connection: Connection,
    _ket: ApiKey,
) -> Result<Json<bool>, Status> {
    match PrivateBoard::update(data.into_inner(), id, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound),
    }
}

#[post("/team/update/<id>", data = "<data>")]
async fn update_team(
    data: Json<BoardUpdate>,
    id: i32,
    connection: Connection,
    _ket: ApiKey,
) -> Result<Json<bool>, Status> {
    match TeamBoard::update(data.into_inner(), id, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound),
    }
}

#[get("/list_delete/<id>")]
async fn delete_list(id: i32, connection: Connection, _ket: ApiKey) -> Result<Json<bool>, Status> {
    match List::delete(id, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound),
    }
}

#[post("/timer/create", data = "<data>")]
async fn timer_create(data: Json<TimerData>, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    if user_id.is_none() {
        return Err(Status::NotFound);
    }
    let name = data.into_inner().name;
    let time = get_time();
    let timer = Timer { id: None, name, user_id: user_id.unwrap(), status: "active".to_owned(), time: 0, start: Some(time)};
    match Timer::create(timer, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        Err(x) => {println!("{:?}", x); Err(Status::NotFound)}
    }
}

#[get("/timer/delete/<id>")]
async fn timer_delete(id: i32, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    match Timer::delete(id, &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound)
    }
}

#[get("/timer/update/<id>")]
async fn timer_update(id: i32, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    let timer = Timer::get_by_id(id, &connection).await;
    if timer.is_err() {
        return Err(Status::NotFound);
    }
    match Timer::update(timer.unwrap(), &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        _ => Err(Status::NotFound)
    }
}

#[get("/timers/get")]
async fn get_timers(connection: Connection, key: ApiKey) -> Result<Json<Vec<Timer>>, Status> {
    let user_id = User::get_username_id(key.0, &connection).await;
    if user_id.is_none() {
        return Err(Status::NotFound);
    }
    match Timer::get_timers(user_id.unwrap(), &connection).await {
        Ok(timers) => Ok(Json(timers)),
        _ => Err(Status::NotFound)
    }
}

#[get("/milestone/get/<id>/<board_type>")]
async fn get_milestones(id: i32, board_type: String, connection: Connection, key: ApiKey) -> Result<Json<Vec<MilestoneResponse>>, Status> {
    match Milestone::get(id, board_type, &connection).await {
        Ok(milestones) => Ok(Json(milestones)),
        _ => Err(Status::NotFound)
    }
}

#[post("/milestone/create", data = "<data>")]
async fn milestone_create(data: Json<Milestone>, connection: Connection, key: ApiKey) -> Result<Json<bool>, Status> {
    match Milestone::create(data.into_inner(), &connection).await {
        Ok(cnt) => Ok(Json(cnt > 0)),
        Err(x) => {Err(Status::NotFound)}
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = Connection::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("can run migrations");

    rocket
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_milestones,
                milestone_create,
                get_logs,
                delete_list,
                update_team,
                update_private,
                update_task,
                get_tasks,
                filter_tasks,
                delete_task,
                delete_team_board,
                login,
                register,
                private_board,
                team_create,
                team_board,
                owned,
                get_private_boards,
                get_team_boards,
                delete_private,
                new_list,
                get_list,
                create_task,
                get_task,
                timer_create,
                timer_delete,
                timer_update,
                get_timers
            ],
        )
        .attach(cors)
        .attach(Connection::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .launch()
        .await?;

    Ok(())
}
