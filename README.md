# Simple CRUD API in Rust based on Axum
Here we provide a small CRUD API in Rust based on the framework axum (https://docs.rs/axum/latest/axum/)
The API can be used to perform CRUD operations on ToDos. This project can be considered as a small project to learn Rust and extend my knowledge about it. Hence, there is still a lot of room for improvements. Just to name a few:

- Add status of ToDos to be able to mark them as done
- Improve error handling (tracing of errors)
- Improve documentation of involved functions, structs, modules, ...
- Unit and Integration Tests
- Authentication

Note that similar projects exists and are taken as a reference for this project:
- https://carlosmv.hashnode.dev/creating-a-rest-api-with-axum-sqlx-rust

## Database
This API uses a Postgre database to store data. We use crate SQLX (https://docs.rs/sqlx/latest/sqlx/) to connect to the database. Note that we also provide scripts to create this database by using podman.
The provided scripts basically support the installation by using the following source:
https://techviewleo.com/how-to-run-postgresql-in-podman-container/?expand_article=1

Provided scripts:
- `./scripts/start_database.sh` 
- `./scripts/prepare_database.sh`

To use these scripts you need to provide the following environmental variables
- DATABASE_URL
- DATABASE_PORT
- DATABASE_USER
- DATABASE_PASSWORD
- DATABASE_EMAIL

To connect to Postgre database via PSQL execute:
`PGPASSWORD=$DATABASE_PASSWORD psql -h localhost -U $DATABASE_USER -p $DATABASE_PORT`


## Execution of Application
To run the application set the environmental variable DATABASE_URL and execute: `cargo run`
Afterwards the following endpoints are available:

- Get: localhost:5000/todo/{id}
- Get: localhost:5000/todos
- Post: localhost:5000/todo/{id}
- Put: localhost:5000/todo/{id}
- Delete: localhost:5000/todo/{id}