# set debug mode
set -x
# set exit on error and find pipe errors
set -eo pipefail

# check if psql is installed
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

# check if sqlx is installed
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Please install sqlx using the following command:"
    echo >&2 "cargo install --version='~0.7' sqlx-cli \
    --no-default-features --features postgres"
    exit 1
fi

#Check if custom user has been set, else default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"

# check if a custom password has been set, default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# check if a custom database name has been set, default to 'newsletter'
DB_NAME="${POSTGRES_DB:=actors}"

# check if a custom port has been set, default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# check if a custom host has been set, default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# If docker container already running, skip
# else, launch a new container
if [[ -n "${RUN_DOCKER}" ]]; then
    # launch using Docker
    docker run \
        -e POSTGRES_USER="${DB_USER}" \
        -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
        -e POSTGRES_DB="${DB_NAME}" \
        -p "${DB_PORT}":5432 \
        -d postgres \
        --name pg_container
        postgres -N 1000
    # -N 1000 to increase the maximum number of connections
    # useful for testing
fi

# docker container may take a few moments to spin up
# keep pinging till connection is made

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" \
    -d "${DB_NAME}" -c '\q'; do
    echo >&2 "Postgres is still unavailable - sleeping"
    sleep 1
done

echo >&2 "Postgress is up and running on port ${DB_PORT}"

# copy the actors.csv file to the docker container
# required for the migration
docker cp data/new_large/actors.csv pg_container:/mnt/actors.csv

# export DB URL
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL

# if RESET_DB is set, drop the database and create a new one
if [[ -n "${RESET_DB}" ]]; then
    echo >&2 "Resetting database"
    sqlx database drop
fi
# make db if not made. sqlx will skip if already made
sqlx database create

# run migrations
sqlx migrate run

# prepare cache
cargo sqlx prepare

echo >&2 "Postgres has been migrated, ready to go!"
