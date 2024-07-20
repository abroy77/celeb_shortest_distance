# set debug mode
set -x
# set exit on error and find pipe errors
set -eo pipefail
# check if sqlx is installed
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Please install sqlx using the following command:"
    echo >&2 "cargo install --version='~0.7' sqlx-cli \
    --no-default-features --features sqlite"
    exit 1
fi


# export DB URL
DATABASE_URL=sqlite:./actors.db
export DATABASE_URL

# if RESET_DB is set, drop the database and create a new one
if [[ -n "${RESET_DB}" ]]; then
    echo >&2 "Resetting database"
    sqlx database drop -y
fi
# make db if not made. sqlx will skip if already made
sqlx database create

# run migrations
sqlx migrate run
echo "migrations complete"
# populate the database
./scripts/populate_db.sh

# prepare cache
cargo sqlx prepare

echo >&2 "DB has been set up! Ready to go!"
