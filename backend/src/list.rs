use crate::db::Connection;
use crate::schema::list;
use diesel::prelude::*;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "list"]
pub struct List {
    pub id: Option<i32>,
    pub name: String,
    pub board: i32,
    pub board_type: String,
}

impl List {
    pub async fn create(list: List, connection: &Connection) -> QueryResult<usize> {
        println!("{:?}", list);
        connection
            .run(|conn| diesel::insert_into(list::table).values(list).execute(conn))
            .await
    }

    pub async fn get(
        board_type: String,
        board_id: i32,
        connection: &Connection,
    ) -> QueryResult<Vec<List>> {
        connection
            .run(move |conn| {
                list::table
                    .filter(list::board_type.eq(board_type))
                    .filter(list::board.eq(board_id))
                    .load::<List>(conn)
            })
            .await
    }

    pub async fn delete(id: i32, connection: &Connection) -> QueryResult<usize> {
        connection
            .run(move |conn| diesel::delete(list::table.filter(list::id.eq(id))).execute(conn))
            .await
    }

    pub async fn delete_by_board(
        board_type: String,
        board: i32,
        connection: &Connection,
    ) -> QueryResult<usize> {
        connection
            .run(move |conn| {
                diesel::delete(
                    list::table
                        .filter(list::board_type.eq(board_type))
                        .filter(list::board.eq(board)),
                )
                .execute(conn)
            })
            .await
    }
}
