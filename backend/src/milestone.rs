use crate::db::Connection;
use crate::list::List;
use crate::schema::log;
use crate::schema::milestone;
use crate::schema::task;
use crate::task::Task;
use crate::utils::matches;
use diesel::prelude::*;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "milestone"]
pub struct Milestone {
    pub id: Option<i32>,
    pub name: String,
    pub board_id: i32,
    pub board_type: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MilestoneResponse {
    pub id: Option<i32>,
    pub name: String,
    pub done: i32,
    pub total: i32,
    pub board_id: i32,
    pub board_type: String,
}

impl Milestone {
    pub async fn create(milestone: Milestone, connection: &Connection) -> QueryResult<usize> {
        connection
            .run(|conn| diesel::insert_into(milestone::table).values(milestone).execute(conn))
            .await
    }


    pub async fn get(board_id: i32, board_type: String, connection: &Connection) -> QueryResult<Vec<MilestoneResponse>> {
        let milestones = connection
            .run(move |conn| milestone::table.filter(milestone::board_id.eq(board_id)).filter(milestone::board_type.eq(board_type)).load::<Milestone>(conn))
            .await?;
        let mut result = Vec::new();
        for milestone in milestones {
            let stats = Self::get_stats(milestone, connection).await?;
            result.push(stats);
        }
        Ok(result)
    }


    pub async fn get_stats(milestone: Milestone, connection: &Connection) -> QueryResult<MilestoneResponse> {
        let board_type = milestone.board_type.clone();
        let lists = List::get(milestone.board_type, milestone.board_id, connection).await?;
        let mut tasks = Vec::new();
        for list in lists {
            let mut tasks_on_list = Task::get(list.id.unwrap(), connection).await?;
            tasks.append(&mut tasks_on_list);
        }
        let id = milestone.id.clone().unwrap();
        let total = tasks.clone().into_iter().filter(|task| matches(task.milestone, id)).count() as i32;
        let done = tasks.into_iter().filter(|task| task.done == 1 && matches(task.milestone,id)).count() as i32;
        Ok(MilestoneResponse{
            total,
            done,
            id: milestone.id,
            board_id: milestone.board_id,
            name: milestone.name,
            board_type
        })
    }
}
