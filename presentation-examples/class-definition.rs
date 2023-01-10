use sqlx::connection::PgConnection;

pub struct Client {
    reqwest: reqwest::Client,
    conn: PgConnection,
}

