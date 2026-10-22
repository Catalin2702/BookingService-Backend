# BookingService – Backend

[English](README.md) · **Italiano**

Backend di BookingService, un sistema di prenotazioni scritto in Rust con [axum](https://github.com/tokio-rs/axum) e [SeaORM](https://www.sea-ql.org/SeaORM/) su PostgreSQL.

> Il progetto è in fase iniziale: per ora il server espone solo gli endpoint di health check.

## Stack

- Rust (edition 2024)
- `axum` 0.8 – server HTTP
- `sea-orm` 2.0 – ORM su PostgreSQL
- `tokio` – runtime asincrono
- `serde` / `serde_json` – serializzazione
- PostgreSQL 18 e Docker Compose

## Endpoint

| Metodo | Percorso     | Descrizione                                                         |
|--------|--------------|---------------------------------------------------------------------|
| `GET`  | `/health`    | Restituisce `200 ok` se il server è attivo                          |
| `GET`  | `/db-health` | Esegue un ping al database: `200 ok`, oppure `503 database unavailable` |

## Struttura del progetto

```
src/
├── main.rs          # entry point: crea lo stato, apre l'indirizzo e avvia il server
├── lib.rs           # moduli del crate
├── env.rs           # configurazione letta dalle variabili d'ambiente
├── state.rs         # AppState: configurazione + pool di connessioni al database
├── routers.rs       # unisce tutti i router
└── routers/
    └── health.rs    # /health e /db-health
```

## Configurazione

La configurazione viene letta solo dalle variabili d'ambiente; il binario non carica file `.env` da solo.

1. Copia il file di esempio e compilalo:

   ```sh
   cp .env.example .env
   ```

2. Crea il file con la password del database (password in chiaro, senza virgolette):

   ```sh
   mkdir -p secrets
   printf '%s' 'la-tua-password' > secrets/postgres_password.txt
   ```

| Variabile                         | Obbligatoria | Descrizione                                                     |
|-----------------------------------|--------------|-----------------------------------------------------------------|
| `BACKEND_HOST`, `BACKEND_PORT`    | sì           | Indirizzo del server (`0.0.0.0` dentro un container)            |
| `DATABASE_HOST`, `DATABASE_PORT`  | sì           | Indirizzo del database (`database` in Compose, `localhost` sull'host) |
| `DATABASE_DB`, `DATABASE_SCHEMA`  | sì           | Nome del database e schema                                      |
| `DATABASE_PROTOCOL`               | sì           | Protocollo di connessione (`postgres`)                          |
| `DATABASE_USER`                   | sì           | Utente del database                                             |
| `DATABASE_PASSWORD_FILE`          | sì\*         | Percorso del file con la password                               |
| `DATABASE_PASSWORD`               | sì\*         | Password, usata se `DATABASE_PASSWORD_FILE` non è impostata     |
| `DATABASE_MAX_CONNECTIONS`        | no           | Default `10`                                                    |
| `DATABASE_MIN_CONNECTIONS`        | no           | Default `0`                                                     |
| `DATABASE_CONNECT_TIMEOUT`        | no           | Secondi, default `10`                                           |
| `DATABASE_ACQUIRE_TIMEOUT`        | no           | Secondi, default `30`                                           |
| `DATABASE_IDLE_TIMEOUT`           | no           | Secondi, default `600`                                          |
| `DATABASE_MAX_LIFETIME`           | no           | Secondi, default `1800`                                         |
| `DATABASE_TEST_BEFORE_ACQUIRE`    | no           | `true`/`false`, default `true`                                  |
| `DATABASE_LOGGING`                | no           | `true`/`false`, default `false`                                 |
| `DATABASE_LOGGING_LEVEL`          | no           | Livello di log delle query SQL, default `info`                  |

\* Ne serve una delle due. Nei file Compose la password è montata come secret e letta tramite `DATABASE_PASSWORD_FILE`.

`.env` contiene i valori di sviluppo, `.env.prod` quelli di produzione. Entrambi sono esclusi da git.

## Avvio

### Con Docker (sviluppo)

```sh
docker compose -f docker-compose.dev.yaml up -d --build
curl http://127.0.0.1:3000/health
```

Avvia i servizi `backend` e `database`. Le porte sono pubblicate solo su `127.0.0.1`.

> Non passare `-p` a `docker compose`: un nome di progetto diverso usa volumi diversi, quindi un database vuoto.

### Sull'host con `cargo run`

Avvia solo il database, poi il server con le variabili esportate:

```sh
docker compose -f docker-compose.dev.yaml up -d database
(set -a; source .env; set +a; \
  DATABASE_HOST=localhost DATABASE_PASSWORD_FILE=secrets/postgres_password.txt \
  cargo run)
```

### Server (produzione)

```sh
docker compose --env-file .env.prod up -d
```

Per ora `docker-compose.yaml` contiene solo il database, raggiungibile tramite un tunnel SSH.

### Accesso al database

```sh
docker exec -it database sh -c 'psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"'
```

## Sviluppo

```sh
cargo build
cargo test
cargo clippy --all-targets
cargo fmt
```
