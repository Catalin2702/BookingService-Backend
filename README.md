# BookingService – Backend

**English** · [Italiano](README.it.md)

Backend for BookingService, a booking system written in Rust with [axum](https://github.com/tokio-rs/axum) and [SeaORM](https://www.sea-ql.org/SeaORM/) on PostgreSQL.

> The project is at an early stage: for now the server only exposes health-check endpoints.

## Stack

- Rust (edition 2024)
- `axum` 0.8 – HTTP server
- `sea-orm` 2.0 – ORM on PostgreSQL
- `tokio` – async runtime
- `serde` / `serde_json` – serialization
- PostgreSQL 18 and Docker Compose

## Endpoints

| Method | Path         | Description                                                      |
|--------|--------------|------------------------------------------------------------------|
| `GET`  | `/health`    | Returns `200 ok` if the server is running                        |
| `GET`  | `/db-health` | Pings the database: `200 ok`, or `503 database unavailable`      |

## Project structure

```
src/
├── main.rs          # entry point: creates the state, binds the address and starts the server
├── lib.rs           # crate modules
├── env.rs           # configuration read from environment variables
├── state.rs         # AppState: configuration + database connection pool
├── routers.rs       # merges all routers
└── routers/
    └── health.rs    # /health and /db-health
```

## Configuration

Configuration is read only from environment variables; the binary does not load `.env` files by itself.

1. Copy the example file and fill it in:

   ```sh
   cp .env.example .env
   ```

2. Create the file containing the database password (raw password, no quotes):

   ```sh
   mkdir -p secrets
   printf '%s' 'your-password' > secrets/postgres_password.txt
   ```

| Variable                          | Required | Description                                                     |
|-----------------------------------|----------|-----------------------------------------------------------------|
| `BACKEND_HOST`, `BACKEND_PORT`    | yes      | Server bind address (`0.0.0.0` inside a container)              |
| `DATABASE_HOST`, `DATABASE_PORT`  | yes      | Database address (`database` in Compose, `localhost` on host)   |
| `DATABASE_DB`, `DATABASE_SCHEMA`  | yes      | Database name and schema                                        |
| `DATABASE_PROTOCOL`               | yes      | Connection protocol (`postgres`)                                |
| `DATABASE_USER`                   | yes      | Database user                                                   |
| `DATABASE_PASSWORD_FILE`          | yes\*    | Path to the password file                                       |
| `DATABASE_PASSWORD`               | yes\*    | Password, used when `DATABASE_PASSWORD_FILE` is not set         |
| `DATABASE_MAX_CONNECTIONS`        | no       | Default `10`                                                    |
| `DATABASE_MIN_CONNECTIONS`        | no       | Default `0`                                                     |
| `DATABASE_CONNECT_TIMEOUT`        | no       | Seconds, default `10`                                           |
| `DATABASE_ACQUIRE_TIMEOUT`        | no       | Seconds, default `30`                                           |
| `DATABASE_IDLE_TIMEOUT`           | no       | Seconds, default `600`                                          |
| `DATABASE_MAX_LIFETIME`           | no       | Seconds, default `1800`                                         |
| `DATABASE_TEST_BEFORE_ACQUIRE`    | no       | `true`/`false`, default `true`                                  |
| `DATABASE_LOGGING`                | no       | `true`/`false`, default `false`                                 |
| `DATABASE_LOGGING_LEVEL`          | no       | Log level for SQL queries, default `info`                       |

\* One of the two is required. In the Compose files the password is mounted as a secret and read through `DATABASE_PASSWORD_FILE`.

`.env` holds the development values, `.env.prod` the production ones. Both are git-ignored.

## Running

### With Docker (development)

```sh
docker compose -f docker-compose.dev.yaml up -d --build
curl http://127.0.0.1:3000/health
```

This starts the `backend` and `database` services. Ports are published on `127.0.0.1` only.

> Do not pass `-p` to `docker compose`: a different project name means different volumes, hence an empty database.

### On the host with `cargo run`

Start only the database, then the server with the variables exported:

```sh
docker compose -f docker-compose.dev.yaml up -d database
(set -a; source .env; set +a; \
  DATABASE_HOST=localhost DATABASE_PASSWORD_FILE=secrets/postgres_password.txt \
  cargo run)
```

### Server (production)

```sh
docker compose --env-file .env.prod up -d
```

For now `docker-compose.yaml` contains only the database, reachable through an SSH tunnel.

### Database access

```sh
docker exec -it database sh -c 'psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"'
```

## Development

```sh
cargo build
cargo test
cargo clippy --all-targets
cargo fmt
```

## License

Released under the [MIT License](LICENSE): you may use, modify and redistribute the code, as long as you keep the copyright notice crediting the author.
