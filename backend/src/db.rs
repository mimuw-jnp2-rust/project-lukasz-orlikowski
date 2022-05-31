use diesel::SqliteConnection;

#[database("sqlite_database")]
pub struct Connection(SqliteConnection);

/*pub type SqlPool = Pool<ConnectionManager<SqliteConnection>>;


static DATABASE_URL: &str = "/home/lukas/Dokumenty/jnp2/project-lukasz-orlikowski/db.sqlite3";

pub fn connect() -> SqliteConnection {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConnection(pub PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.*/
/*#[rocket::async_trait]
impl<'a> FromRequest<'a> for Connection {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}*/

// For the convenience of using an &Connection as an &MysqlConnection.
/*impl Deref for Connection {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}*/
