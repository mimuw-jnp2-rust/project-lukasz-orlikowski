use rocket::http::{Status};
use types::{Credentials, TokenResponse };
use db::Connection;
use rocket::serde::json::Json;
use self::auth::crypto::sha2::Sha256;
use self::auth::jwt::{
    Header,
    Registered,
    Token,
};
use user::User;

pub mod types;
pub mod auth;
pub mod db;
pub mod user;
pub mod schema;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate rocket_sync_db_pools;

use rocket::{Rocket, Build, Response, Request};
use rocket::fairing::AdHoc;

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

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn =  Connection::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("can run migrations");

    rocket
}

#[catch(404)]
fn not_found(req: &Request) { 
    println!("{:?}", req);
 }


 use std::error::Error;
use std::str::FromStr;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, AllowedMethods, CorsOptions};


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
        .mount("/", routes![login, register])
        .attach(cors).attach(Connection::fairing()).attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .launch()
        .await?;

    Ok(())
}
