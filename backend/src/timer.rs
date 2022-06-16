use crate::schema::timer;
use crate::db::Connection;
use crate::utils::get_time;
use diesel::prelude::*;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "timer"]
pub struct Timer {
    pub id: Option<i32>,
    pub name: String,
    pub user_id: i32,
    pub status: String,
    pub time: i32,
    pub start: Option<i32>
}

impl Timer {
    pub async fn create(timer: Timer, connection: &Connection) -> QueryResult<usize> {
        connection
            .run(|conn| {
                diesel::insert_into(timer::table)
                    .values(timer)
                    .execute(conn)
            })
            .await
    }

    pub async fn update(timer: Timer, connection: &Connection) -> QueryResult<usize> {
        let status = if timer.status == "active" {"stopped"} else {"active"};
        let time = if timer.status == "active" {timer.time + get_time() - timer.start.unwrap()} else {timer.time};
        connection
        .run(move |conn| {
            diesel::update(timer::table.filter(timer::id.eq(timer.id)))
                .set((timer::status.eq(status), timer::time.eq(time), timer::start.eq(get_time())))
                .execute(conn)
        })
        .await
    }

    pub async fn get_timers(user_id: i32, connection: &Connection) -> QueryResult<Vec<Timer>> {
        connection
            .run(move |conn| timer::table.filter(timer::user_id.eq(user_id)).load::<Timer>(conn))
            .await
    }

    pub async fn get_by_id(id: i32, connection: &Connection) -> QueryResult<Timer> {
        connection
        .run(move |conn| timer::table.filter(timer::id.eq(id)).first(conn))
        .await
    }

    pub async fn delete(id: i32, connection: &Connection) -> QueryResult<usize> {
        connection
        .run(move |conn| {
            diesel::delete(timer::table.filter(timer::id.eq(id))).execute(conn)
        })
        .await
    }
}