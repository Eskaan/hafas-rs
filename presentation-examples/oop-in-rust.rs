use sqlx::connection::PgConnection;

pub struct Client {
    /// A reqwest client used to make web requests.
    reqwest: reqwest::Client,
    /// A database connection used to insert data
    conn: PgConnection,
}

impl Client {
    pub async fn make_request(&mut self, url: &str) {
        self.reqwest
            .post(url)
            .send()
            .await
            .unwrap();
    }
}

impl From<PgConnection> for Client {
    fn from(value: PgConnection) -> Client {
        Client {
            reqwest: reqwest::Client::default(),
            conn: value,
        }
    }
}

impl Client {
    pub fn new() -> Client {
        Client {
            reqwest: reqwest::Client::default(),
            conn: database::connect(),
        }
    }
}
