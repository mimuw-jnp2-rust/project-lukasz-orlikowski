use diesel::SqliteConnection;

#[database("sqlite_database")]
pub struct Connection(SqliteConnection);
