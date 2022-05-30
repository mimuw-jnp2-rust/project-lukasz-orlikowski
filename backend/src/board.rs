use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::schema::private_board;
use crate::schema::team_board;
use crate::types::BoardUpdate;
use rocket::serde::{Deserialize, Serialize};
use crate::db::Connection;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "private_board"]
pub struct PrivateBoard {
    pub id: Option<i32>,
    pub name: String,
    pub owner: i32
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "team_board"]
pub struct TeamBoard {
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

    pub async fn delete(id: i32, connection: &Connection) ->QueryResult<usize> {
        connection.run(move |conn| {
            diesel::delete(private_board::table.filter(private_board::id.eq(id))).execute(conn)
        }).await
    }

    pub async fn update(board: BoardUpdate, id: i32, connection: &Connection) -> QueryResult<usize> {
        connection.run(move |conn| {
            diesel::update(private_board::table.filter(private_board::id.eq(id)))
                .set(private_board::name.eq(board.name))
                .execute(conn)
           }).await
    }
}

impl TeamBoard {
    pub async fn create(board: TeamBoard, connection: &Connection) -> QueryResult<usize> {
        connection.run(|conn| {
         diesel::insert_into(team_board::table)
             .values(board)
             .execute(conn)
        }).await
     }


    pub async fn delete(id: i32, connection: &Connection) ->QueryResult<usize> {
        connection.run(move |conn| {
            diesel::delete(team_board::table.filter(team_board::id.eq(id))).execute(conn)
        }).await
    }

    pub async fn update(board: BoardUpdate, id: i32, connection: &Connection) -> QueryResult<usize> {
        connection.run(move |conn| {
            diesel::update(team_board::table.filter(team_board::id.eq(id)))
                .set(team_board::name.eq(board.name))
                .execute(conn)
           }).await
    }
}