use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use diesel::connection;
use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::schema::task;
use rocket::serde::{Deserialize, Serialize};
use crate::db::Connection;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "task"]
pub struct Task {
    pub id: Option<i32> ,
    pub name: String,
    pub list: i32,
    pub note: Option<String>,
    pub place: Option<String>,
    pub members: Option<String>,
}

impl Task {
    pub async fn create(task: Task, connection: &Connection) -> QueryResult<usize>{
        connection.run(|conn| {
            diesel::insert_into(task::table)
                .values(task)
                .execute(conn)
           }).await
    }

    pub async fn get(id: i32, connection: &Connection) -> QueryResult<Vec<Task>>{
        connection.run(move |conn| {
            task::table.filter(task::list.eq(id)).load::<Task>(conn)
        }).await
    }

    pub async fn delete(id: i32, connection: &Connection) ->QueryResult<usize> {
        connection.run(move |conn| {
            diesel::delete(task::table.filter(task::id.eq(id))).execute(conn)
        }).await
    }
}