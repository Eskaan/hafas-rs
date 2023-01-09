CREATE SCHEMA lookup_data;

CREATE TABLE lookup_data.raw_data (
    jid int PRIMARY KEY,
    raw json NOT NULL
);

CREATE TABLE lookup_data.locations (
    eva int PRIMARY KEY,
    name text NOT NULL,
    x int NOT NULL,
    y int NOT NULL,
    z int
);
CREATE TABLE lookup_data.operators (
    name text PRIMARY KEY
);
CREATE TABLE lookup_data.train_types (
    cat_code int NOT NULL,
    cat_out text NOT NULL,
    PRIMARY KEY (cat_code, cat_out)
);
CREATE TYPE lookup_data.operation_dates AS (
    dates date[],
    from_loc int,
    to_loc int,
    info text
);
CREATE TYPE lookup_data.scheduled_stop AS (
    eva int,
    scheduled_arrival time,
    scheduled_departure time
);
CREATE TABLE lookup_data.trips (
    jid int PRIMARY KEY,
    op_days lookup_data.operation_dates[] NOT NULL, 
    cat_code smallint NOT NULL,
    cat_out text NOT NULL,
    FOREIGN KEY (cat_code, cat_out) REFERENCES lookup_data.train_types (cat_code, cat_out),
    name text NOT NULL,
    route text NOT NULL,
    id int NOT NULL,
    admin text NOT NULL,
    operator text REFERENCES lookup_data.operators (name),
    stops lookup_data.scheduled_stop[] NOT NULL
);

CREATE TABLE lookup_data.location_counts (
    eva int REFERENCES lookup_data.locations (eva),
    cat_code smallint NOT NULL,
    cat_out text NOT NULL,
    FOREIGN KEY (cat_code, cat_out) REFERENCES lookup_data.train_types (cat_code, cat_out),
    PRIMARY KEY (eva, cat_code, cat_out),
    name text NOT NULL,
    count int NOT NULL
);


CREATE FUNCTION lookup_data.get_evas(lookup_data.scheduled_stop[]) RETURNS int[] AS
$func$
    SELECT array_agg(vals)
    FROM (select (unnest($1)).eva as vals) AS stop_evas;
$func$ LANGUAGE sql STABLE;
