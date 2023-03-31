# Logical Replication

A project to play with and test Postgres [logical replication](https://www.postgresql.org/docs/current/logical-replication.html) using Rust.

## Why

[Logical replication](https://www.postgresql.org/docs/current/logical-replication.html) gives the ability to subscribe to the Postgres write-ahead-log messages and decode them into usable (and transactional) data. There are many uses for this functionality, for example:

- a web server could store (and invalidate) a local cache of a table in a database to prevent a database round-trip.
- a notification could be sent to a user as a result of an action by another user connected to a different web server instance.

[Logical replication](https://www.postgresql.org/docs/current/logical-replication.html) is lower level than the Postgres [LISTEN](https://www.postgresql.org/docs/current/sql-listen.html) functionality, causes [no performance impact](https://reorchestrate.com/posts/debezium-performance-impact/) and does not require the user to choose which tables to listen to.

## What

The main test is in [types/mod.rs](./src/types/mod.rs).

This test attempts to perform deterministic simulation by first attaching the `logicalreplication` listener to an empty database then:

1. Deterministically produce random batches of transactions against an in-memory representation of the table.
2. Applying the batched transactions to the Postgres database.
3. Listening to the logical replication stream and trying to apply them to a second in-memory representation of the table.
4. Stopping the test after `n` iterations and then testing that all three representations align.

## How

1. Start postgres with logical replication mode - see the `docker-compose.yaml` and the `Dockerfile` for configuration.
2. Run `cargo install sqlx-cli` to set up the [sqlx](https://github.com/launchbadge/sqlx) command line utility to allow database migrations.
3. Run `sqlx migrate run` to set up the intial database.
4. Run `cargo test`.

## DEMO

before starting if you're running your postgres instance on a different port make sure 
to change the file [replication.rs](src%2Freplication.rs) `db_config` variable to make it match your port 
1. run `docker-compose up` to start the postgres container locally
2. Run `cargo install sqlx-cli` to set up the [sqlx](https://github.com/launchbadge/sqlx) command line utility to allow database migrations.
3. Run `sqlx migrate run` to set up the intial database.
4. Run `export DATABASE_URL="postgres://postgres:password@localhost:5432/postgres"` (make sure to update the port accordantly)
5. Run `cargo run`. 
6. Run `docker exec -it <CONTAINER_ID> psql -U postgres` to connect to your local postgres container [you could grab the container ID using `docker ps -f name=postgres`]
7. Run the following to create a new table
   ```
   CREATE TABLE demo_postgres_cdc (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    email TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE);
   ```
8. Run the following to Insert 20 random records into the demo_table
   ```
   INSERT INTO demo_postgres_cdc (name, age, email, is_active)
   SELECT
   'User ' || generate_series(1, 20) AS name,
   (random() * 50 + 18)::integer AS age,
   'user' || generate_series(1, 20) || '@example.com' AS email,
   (random() > 0.5) AS is_active;
    ```
9. Run the following to Update a random record in the demo_table
    ```
    UPDATE demo_postgres_cdc
    SET                                                                            
        name = 'Updated Name',
        age = 30,
        email = 'updated@example.com',
        is_active = NOT is_active
    WHERE id = (SELECT id FROM demo_postgres_cdc ORDER BY random() LIMIT 1);
    ```

10. Run the following to Delete a random record from the demo_table
     ```
    DELETE FROM demo_postgres_cdc
     WHERE id = (SELECT id FROM demo_postgres_cdc ORDER BY random() LIMIT 1);
     ```


## Further

Ideas of what would be helpful:

- It would be good to build a [procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html) similar to [structmap](https://crates.io/crates/structmap) which automates the generation of applying what is received from the logical decoding (effectively a vector of hashmaps) directly to structs.

- This version deliberately chooses [decoderbufs](https://github.com/debezium/postgres-decoderbufs) but work could be done to ensure it works with [wal2json](https://github.com/eulerto/wal2json) too and that output data is standardised.

## Acknowledgements

Thank you to:

- `rust-postgres`: https://github.com/sfackler/rust-postgres/issues/116
- [Materialize](https://materialize.com/)'s fork of `rust-postgres` with the patches required to support logical decoding: https://github.com/MaterializeInc/rust-postgres
- `postgres-decoderbufs`: https://github.com/debezium/postgres-decoderbufs
- this example: https://github.com/debate-map/app/blob/afc6467b6c6c961f7bcc7b7f901f0ff5cd79d440/Packages/app-server-rs/src/pgclient.rs
