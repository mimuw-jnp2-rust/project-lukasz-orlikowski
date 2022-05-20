use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::schema::private_board;
use rocket::serde::{Deserialize, Serialize};
use crate::db::Connection;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "private_board"]
pub struct PrivateBoard {
    pub id: Option<i32>,
    pub name: String,
    pub owner: i32
}

impl PrivateBoard {
    pub async fn create(board: PrivateBoard, connection: &Connection) -> QueryResult<usize> {
        connection.run(|conn| {
         diesel::insert_into(private_board::table)
             .values(board)
             .execute(conn)
        }).await
     }
}