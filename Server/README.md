# Server

This directory includes the server code. The server runs on Rust using Actix v4 and a Postgres db.

## Running

To run the server you need to specify some environment variables:
- `PG.PASSWORD`: Postgres database password for `postgres` user
- `PG.HOST`: Postgres server address
- `PG.DBNAME`: Postgres server database to use
- `API_KEY`: Key to allow access to the API

These should be declared in the `.env` file in the project root.

Run with `cargo run --release`

## Log file

The server will generate a log file. The path for this is specified in [`src/main.rs`](src/main_). Each line of the log file has the following format

```
[datetime] REQUEST client method path
[datetime] RESPONSE client method path response
[datetime] DB db_log
```

- `datetime` is the date-time as defined by RFC 3339/ISO 8601 (`2023-02-01T15:34:12.351877+00:00`)
- `client` is the client address (`IP:port`)
- `method` is the HTTP method (GET, POST, etc.)
- `path` is the request path (`api/submit` etc.)
- `response` is the returned response code

An example line might look like the following

```
[2023-02-01T15:34:12.351877+00:00] 127.0.0.1:8080 POST /api/submit 200 3
```
