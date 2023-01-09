use sqlx::{Connection, PgConnection};

/// Connect to the local database.
/// The database must be at `postgres://postgres@localhost/db-statistics`.
pub async fn connect() -> PgConnection {
    PgConnection::connect("postgres://postgres@localhost/db-statistics")
        .await
        .unwrap()
}
