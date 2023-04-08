#!/bin/sh

if [ "$DATABASE" = "devops_hobbies_db" ]
then
    echo "Waiting for devops_hobbies_db..."

    while ! nc -z $SQL_HOST $SQL_PORT; do
      sleep 0.1
    done

    echo "PostgreSQL started"
fi

exec "$@"