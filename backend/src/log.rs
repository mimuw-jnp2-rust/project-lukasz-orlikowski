use crate::db::Connection;
use crate::schema::log;
use crate::task::Task;
use crate::utils::get_date;
use diesel::prelude::*;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "log"]
pub struct Log {
    pub id: Option<i32>,
    pub name: String,
    pub list: i32,
    pub note: Option<String>,
    pub place: Option<String>,
    pub members: Option<String>,
    pub timestamp: String,
    pub action: String,
    pub task_id: i32,
    pub deadline: String,
    pub subtasks: String,
    pub points: i32,
    pub tags: String
}

impl Log {
    pub async fn create(log: Log, connection: &Connection) -> QueryResult<usize> {
        connection
            .run(|conn| diesel::insert_into(log::table).values(log).execute(conn))
            .await
    }


    pub async fn get(id: i32, connection: &Connection) -> QueryResult<Vec<Log>> {
        connection
            .run(move |conn| log::table.filter(log::task_id.eq(id)).order((log::timestamp.desc())).load::<Log>(conn))
            .await
    }

    pub fn from_task(task: Task, id: i32, action: String) -> Log {
        Log { id: None, name: task.name, list: task.list, note: task.note, place: task.place, members: task.members, timestamp: get_date(), action, task_id: id, deadline: task.deadline, subtasks: task.subtasks, points: task.points, tags: task.tags }
    }
}
