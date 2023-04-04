# Multi stage docker containers

This is an example of a Multi-Stage builds that runs another programming language application (go package) inside other container (Python Flask).
This project is a Flask application that demonstrates connecting to a Postgresql database and running a Go script using subprocess in Python. \
The application is containerized with Docker and can be run using docker-compose.


## Prerequisites
    Docker
    Docker Compose

## Getting started
1. Build the Docker container:
    `docker-compose build`

2. Start the container: `docker-compose up -d`

#### -- OR --

    docker-compose up [-d] --build 

The application should now be running on http://localhost:8088\
You can also check the status of the database on http://localhost:8088/db-status

____

## Configuration

The application reads environment variables from a .env file. The following variables must be set:

    DB_HOST: The hostname of the Postgresql database
    DB_PORT: The port of the Postgresql database
    POSTGRES_DB: The name of the database to connect to
    POSTGRES_USER: The username to authenticate with
    POSTGRES_PASSWORD: The password to authenticate with

## Project Structure

    .
    ├── docker-compose.yml
    ├── Dockerfile
    ├── entrypoint.sh
    ├── README.md
    ├── .env
    └── webapp
        ├── app.py
        ├── requirements.txt
        ├── .env
        └── scripts
            └── helloworld-go

_

`docker-compose.yml`: Docker Compose configuration file.\
`Dockerfile`: Docker configuration file.\
`entrypoint.sh`: Entrypoint script that runs on container startup (Check the PostgreSQL health).\
`webapp/app.py`: Flask application code.\
`webapp/requirements.txt`: Python requirements file.\
`webapp/scripts/helloworld-go`: Go application that is run inside the Flask application.