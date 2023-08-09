#!/bin/bash

echo "*** Collect static files ***"
python manage.py collectstatic --noinput

echo "*** Apply database migrations ***"
python manage.py makemigrations

echo '*** migrate to database ***'
python manage.py migrate
python manage.py migrate --database=log

# create system user
# python manage.py create_system_user #TODO:
