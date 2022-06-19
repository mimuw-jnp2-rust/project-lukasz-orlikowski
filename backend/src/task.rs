use std::fmt::format;

use crate::db::Connection;
use crate::log::Log;
use crate::schema::task;
use crate::types::TaskFilter;
use diesel::prelude::*;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug, Clone)]
#[table_name = "task"]
#[changeset_options(treat_none_as_null="true")]
pub struct Task {
    pub id: Option<i32>,
    pub name: String,
    pub list: i32,
    pub note: Option<String>,
    pub place: Option<String>,
    pub members: Option<String>,
    pub deadline: String,
    pub subtasks: String,
    pub points: i32,
    pub tags: String,
    pub done: i32, // In sqlite we don't have boolean types
    pub milestone: Option<i32>
}

impl Task {
    pub async fn create(task: Task, connection: &Connection) -> QueryResult<usize> {
        let task_clone = task.clone();
        let res = connection
            .run(|conn| diesel::insert_into(task::table).values(task_clone).execute(conn))
            .await;
        if res.is_ok() {
            let id = connection.run(|conn| diesel::select(last_insert_rowid)
            .get_result::<i32>(conn)).await;
            let log = Log::from_task(task, id.unwrap(), "created".to_owned());
            let _ = Log::create(log, connection).await;
        }
        res
    }

    pub async fn update(task: Task, connection: &Connection) -> QueryResult<usize> {
        let task_clone = task.clone();
        let res = connection
            .run(|conn| {
                diesel::update(task::table.filter(task::id.eq(task_clone.id.unwrap())))
                    .set(task_clone)
                    .execute(conn)
            })
            .await;
        if res.is_ok() {
            let log = Log::from_task(task.clone(), task.id.unwrap(), "updated".to_owned());
            let _ = Log::create(log, connection).await;
        }
        res
    }

    pub async fn get(id: i32, connection: &Connection) -> QueryResult<Vec<Task>> {
        connection
            .run(move |conn| task::table.filter(task::list.eq(id)).load::<Task>(conn))
            .await
    }

    pub async fn filter(id: i32, data: TaskFilter, connection: &Connection) -> QueryResult<Vec<Task>> {

        connection
            .run(move |conn| {
                let mut query = task::table.into_boxed();
        query = query.filter(task::name.like(format!("%{}%", data.name)));
        query = query.filter(task::place.like(format!("%{}%", data.place)));
        
        if let Some(mini) = data.points_min {
            query = query.filter(task::points.ge(mini));
        }
        if let Some(maxi) = data.points_max {
            query = query.filter(task::points.le(maxi));
        }
        
        if data.deadline_start != "" {
            query = query.filter(task::deadline.ge(data.deadline_start));
        }
        if data.deadline_end != "" {
            query = query.filter(task::deadline.le(data.deadline_end));
        }

        for member in data.members.split(";") {
            query = query.filter(task::members.like(format!("%{}%", member)));
        }

        for tag in data.tags.split(";") {
            query = query.filter(task::tags.like(format!("%{}%", tag)));
        }
                query.filter(task::list.eq(id)).load::<Task>(conn)})
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
