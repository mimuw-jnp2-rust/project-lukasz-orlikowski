use crate::db::Connection;
use crate::schema::task;
use diesel::prelude::*;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "task"]
pub struct Task {
    pub id: Option<i32>,
    pub name: String,
    pub list: i32,
    pub note: Option<String>,
    pub place: Option<String>,
    pub members: Option<String>,
    pub deadline: String,
    pub subtasks: String,
}

impl Task {
    pub async fn create(task: Task, connection: &Connection) -> QueryResult<usize> {
        connection
            .run(|conn| diesel::insert_into(task::table).values(task).execute(conn))
            .await
    }

    pub async fn update(task: Task, connection: &Connection) -> QueryResult<usize> {
        connection
            .run(|conn| {
                diesel::update(task::table.filter(task::id.eq(task.id.unwrap())))
                    .set(task)
                    .execute(conn)
            })
            .await
    }

    pub async fn get(id: i32, connection: &Connection) -> QueryResult<Vec<Task>> {
        connection
            .run(move |conn| task::table.filter(task::list.eq(id)).load::<Task>(conn))
            .await
    }

    pub async fn delete(id: i32, connection: &Connection) -> QueryResult<usize> {
        connection
            .run(move |conn| diesel::delete(task::table.filter(task::id.eq(id))).execute(conn))
            .await
    }

    pub async fn get_single(id: i32, connection: &Connection) -> QueryResult<Task> {
        connection
            .run(move |conn| task::table.filter(task::id.eq(id)).first(conn))
            .await
    }
}
