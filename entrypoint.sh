#!/bin/bash
set -e

# Wait for database to be ready
echo "Waiting for database..."
until PGPASSWORD=xD psql -h db -U hehe -d player_db -c '\q' 2>/dev/null; do
  echo "Database is unavailable - sleeping"
  sleep 1
done

echo "Database is up - running migrations"
sqlx migrate run

echo "Starting application"
exec ./player
