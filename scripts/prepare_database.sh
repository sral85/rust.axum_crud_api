if [ -z "$DATABASE_URL" ]; then
    echo "Need to set DATABASE_URL to postgre database"
    exit 1
fi  

sqlx database create
sqlx migrate run