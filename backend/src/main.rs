use rocket::futures::future::join_all;
use rocket::http::{Status};
use types::{Credentials, TokenResponse };
use db::Connection;
use rocket::serde::json::Json;
use crate::auth::ApiKey;
use crate::types::{PrivateBoardData, TeamData, TeamBoardData};
use board::{PrivateBoard, TeamBoard};
use team::Team;

use self::auth::crypto::sha2::Sha256;
use self::auth::jwt::{
    Header,
    Registered,
    Token,
};
use user::User;

pub mod types;
pub mod team;
pub mod auth;
pub mod db;
pub mod user;
pub mod schema;
pub mod board;
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
        .mount("/", routes![login, register, private_board, team_create, team_board])
        .attach(cors).attach(Connection::fairing()).attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .launch()
        .await?;

    Ok(())
}
