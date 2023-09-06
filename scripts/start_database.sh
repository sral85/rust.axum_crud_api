# Use this script to start a database
## Create Pod
podman pod create --name postgre-sql -p 9876:80 -p 5555:5432

## Run PGAdmin
podman run --pod postgre-sql \
  -e PGADMIN_DEFAULT_EMAIL=$DATABASE_EMAIL \
  -e PGADMIN_DEFAULT_PASSWORD=$DATABASE_PASSWORD  \
  --name pgadmin \
  -d docker.io/dpage/pgadmin4:latest 

## Run Postgre
podman run --volume ~/Dokumente/Programming/Mounts/postgredb/:/var/lib/postgresql/data --name db --pod=postgre-sql -d \
  -e POSTGRES_USER=$DATABASE_USER \
  -e POSTGRES_PASSWORD=$DATABASE_PASSWORD \
  docker.io/library/postgres:14 
  




