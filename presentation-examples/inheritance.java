import sqlx.connection.PgConnection;
import reqwest.Client;

public class Client implements From<PgConnection> {
    private Client reqwest;
    private PgConnection conn;

    public Client (PgConnection conn) {
        this.reqwest = new reqwest.Client();
        this.conn = conn;
    };

    public Client from(PgConnection value) {
        return new Client(value);
    }
}
