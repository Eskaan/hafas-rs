use sqlx::connection::PgConnection;

pub struct Client {
    reqwest: reqwest::Client,
    conn: PgConnection,
}

impl Client {
    pub fn make_request(&self, url: String) -> String {
        let response: String = self.reqwest
            .post(&url)
            .send()
            .text;
        
        return text;
    }
}

