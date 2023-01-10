import sqlx.connection.PgConnection;
import reqwest.Client;

public class Client {
    private Client reqwest;
    private PgConnection conn;

    public String make_request(String url) {
        String response = this.reqwest
            .post(url)
            .send()
            .text;
        
        return text;
    }
}

