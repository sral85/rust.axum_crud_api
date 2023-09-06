# Simple CRUD API in Rust based on Axum
Here we want to provide a small CRUD API based on axum.

## Open Tasks / Possible Improvements
- Create git repository
- Improve error handling
- Add status to ToDos
- Improve documentation
- Unit and Integration Tests
- Authentication
- tracing of errors

## How to install postgres database
Taken from https://techviewleo.com/how-to-run-postgresql-in-podman-container/?expand_article=1

## Create and prepare database
./start_database.sh 
./prepare_database.sh

## Set environmental variables
- DATABASE_URL
- DATABASE_PORT
- DATABASE_USER
- DATABASE_PASSWORD
- DATABASE_EMAIL

## Connect to Postgre Database via PSQL
PGPASSWORD=$DATABASE_PASSWORD psql -h localhost -U $DATABASE_USER -p $DATABASE_PORT