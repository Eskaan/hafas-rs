use sqlx::connection::PgConnection;

pub struct Client {
    reqwest: reqwest::Client,
    conn: PgConnection,
}

impl From<PgConnection> for Client {
    fn from(value: PgConnection) -> Client {
        Client {
            reqwest: reqwest::Client::default(),
            conn: value,
        }
    }
}
