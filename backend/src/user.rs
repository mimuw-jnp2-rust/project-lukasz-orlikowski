use crate::board::PrivateBoard;
use crate::db::Connection;
use crate::schema::private_board;
use crate::schema::users;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use diesel::SqliteConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn create(user: User, connection: &Connection) -> QueryResult<usize> {
        connection
            .run(|conn| {
                diesel::insert_into(users::table)
                    .values((
                        users::username.eq(user.username),
                        users::password.eq(user.password),
                    ))
                    .execute(conn)
            })
            .await
    }

    pub fn read(id: i32, connection: &SqliteConnection) -> QueryResult<Vec<User>> {
        if id != 0 {
            users::table.find(id).load::<User>(connection)
        } else {
            users::table.order(users::id).load::<User>(connection)
        }
    }

    pub async fn by_username_and_password(
        username_: String,
        password_: String,
        connection: &Connection,
    ) -> Option<User> {
        connection
            .run(|conn| {
                let res = users::table
                    .filter(users::username.eq(username_))
                    .filter(users::password.eq(password_))
                    .order(users::id)
                    .first(conn);
                match res {
                    Ok(user) => Some(user),
                    Err(_) => None,
                }
            })
            .await
    }

    pub async fn get_private_boards(
        user_id: i32,
        connection: &Connection,
    ) -> QueryResult<Vec<PrivateBoard>> {
        connection
            .run(move |conn| {
                private_board::table
                    .filter(private_board::owner.eq(user_id))
                    .load::<PrivateBoard>(conn)
            })
            .await
    }

    pub fn update(id: i32, user: User, connection: &SqliteConnection) -> bool {
        diesel::update(users::table.find(id))
            .set(&user)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
        diesel::delete(users::table.find(id))
            .execute(connection)
            .is_ok()
    }

    pub async fn get_username_id(username: String, connection: &Connection) -> Option<i32> {
        connection
            .run(|conn| {
                let res: Result<User, Error> = users::table
                    .filter(users::username.eq(username))
                    .order(users::id)
                    .first(conn);
                match res {
                    Ok(user) => Some(user.id.unwrap()), // Id is not none here due to primary key constraint
                    Err(_) => None,
                }
            })
            .await
    }
}
