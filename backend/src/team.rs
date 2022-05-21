use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel::result::Error;
use crate::schema::{team, team_user};
use rocket::serde::{Deserialize, Serialize};
use crate::db::Connection;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "team"]
pub struct Team {
    pub id: Option<i32>,
    pub name: String,
    pub owner: i32
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "team_user"]
pub struct TeamUser {
    pub id: Option<i32>,
    pub team: i32,
    pub user: i32
}

impl Team {
    pub async fn create (members: Vec<Option<i32>>, name: String, owner: i32, connection: &Connection) -> QueryResult <usize> {
        let copied_name = name.clone();
        let rows_affected = connection.run(move |conn| {
            diesel::insert_into(team::table)
                .values((team::name.eq(name), team::owner.eq(owner)))
                .execute(conn)
           }).await;

        let team_id = Team::get_name_id(copied_name, connection).await;
        
        for member in members {
            Team::add(team_id, member, connection).await;
        }

        rows_affected
    }

    pub async fn get_name_id(name: String, connection: &Connection) -> Option<i32> {
        connection.run(|conn| { 
            let res: Result<Team, Error> = team::table
            .filter(team::name.eq(name))
            .order(team::id)
            .first(conn);
            match res {
                Ok(team) =>{Some(team.id.unwrap()) }, // Id is not none here due to primary key constraint
                Err(_) => {
                    None
                }
            }
        }).await
    }

    pub async fn get_owned(user_id: i32, connection: &Connection) -> QueryResult<Vec<Team>> {
        connection.run(move |conn| {
            team::table.filter(team::owner.eq(user_id)).load::<Team>(conn)
        }).await
    }

    pub async fn add (team_id: Option<i32>, user_id: Option<i32>, connection: &Connection) -> () {
        match (team_id, user_id) {
            (Some(t_id), Some(u_id)) => {
                connection.run(move |conn| {
                    let res = diesel::insert_into(team_user::table)
                        .values((team_user::user.eq(u_id), team_user::team.eq(t_id)))
                        .execute(conn);
                   }).await;
            }
            _ => {()}
        }
    }
}