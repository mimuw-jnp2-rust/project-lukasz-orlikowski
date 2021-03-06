use crate::board::TeamBoard;
use crate::db::Connection;
use crate::schema::team_board;
use crate::schema::{team, team_user};
use crate::types::TeamBoardWithName;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "team"]
pub struct Team {
    pub id: Option<i32>,
    pub name: String,
    pub owner: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "team_user"]
pub struct TeamUser {
    pub id: Option<i32>,
    pub team: i32,
    pub user: i32,
}

impl Team {
    pub async fn create(
        members: Vec<Option<i32>>,
        name: String,
        owner: i32,
        connection: &Connection,
    ) -> QueryResult<usize> {
        let copied_name = name.clone();
        let rows_affected = connection
            .run(move |conn| {
                diesel::insert_into(team::table)
                    .values((team::name.eq(name), team::owner.eq(owner)))
                    .execute(conn)
            })
            .await;

        let team_id = Team::get_name_id(copied_name, owner, connection).await;

        for member in members {
            Team::add(team_id, member, connection).await;
        }

        rows_affected
    }

    pub async fn get_name_id(name: String, owner: i32, connection: &Connection) -> Option<i32> {
        connection
            .run(move |conn| {
                let res: Result<Team, Error> = team::table
                    .filter(team::name.eq(name))
                    .filter(team::owner.eq(owner))
                    .order(team::id)
                    .first(conn);
                match res {
                    Ok(team) => Some(team.id.unwrap()), // Id is not none here due to primary key constraint
                    Err(_) => None,
                }
            })
            .await
    }

    pub async fn has_access(
        team: i32,
        user: i32,
        connection: &Connection,
    ) -> Result<TeamUser, Error> {
        connection
            .run(move |conn| {
                team_user::table
                    .filter(team_user::team.eq(team))
                    .filter(team_user::user.eq(user))
                    .order(team_user::id)
                    .first(conn)
            })
            .await
    }

    pub async fn get_owned(user_id: i32, connection: &Connection) -> QueryResult<Vec<Team>> {
        connection
            .run(move |conn| {
                team::table
                    .filter(team::owner.eq(user_id))
                    .load::<Team>(conn)
            })
            .await
    }

    pub async fn add(team_id: Option<i32>, user_id: Option<i32>, connection: &Connection) {
        if let (Some(t_id), Some(u_id)) = (team_id, user_id) {
            connection
                .run(move |conn| {
                    let _res = diesel::insert_into(team_user::table)
                        .values((team_user::user.eq(u_id), team_user::team.eq(t_id)))
                        .execute(conn);
                })
                .await;
        }
    }

    pub async fn get_teams_boards(
        user_id: i32,
        connection: &Connection,
    ) -> QueryResult<Vec<TeamBoardWithName>> {
        connection
            .run(move |conn| {
                let team_user = team_user::table
                    .filter(team_user::user.eq(user_id))
                    .load::<TeamUser>(conn)?;
                let mut teams = Vec::<Team>::new();
                for team in team_user {
                    let mut tmp = team::table
                        .filter(team::id.eq(team.team))
                        .load::<Team>(conn)?;
                    teams.append(&mut tmp);
                }
                let mut boards = Vec::<TeamBoardWithName>::new();
                for team in teams {
                    let tmp = team_board::table
                        .filter(team_board::owner.eq(team.id.unwrap()))
                        .load::<TeamBoard>(conn)?;
                    let mut board_team = TeamBoardWithName::new(tmp, team.name);
                    boards.append(&mut board_team);
                }
                Ok(boards)
            })
            .await
    }
}
