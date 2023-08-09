![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![FastAPI](https://img.shields.io/badge/FastAPI-005571?style=for-the-badge&logo=fastapi)

# Dockerfile for Beginners

This project contains a simple and practical example of a Dockerfile to help beginners learn how to use Docker.

## Prerequisites

- Docker installed on your OS. [Docker Installation](https://docs.docker.com/engine/install/)
- Basic knowledge of the Linux command line.

## Each Line Explanation in Dockerfile

- ``FROM tiangolo/uvicorn-gunicorn-fastapi:python3.9-slim``: Use the official fastapi image as our base image. We're using the slim version to keep the image small.

- ``WORKDIR /app``: Set the working directory in the container to `/app`.

- ``COPY requirements.txt .``: Copy the `requirements.txt` file from the current directory to the container which is `/app` directory, because in the previous step we have used `WORKDIR` this directive makes `/app` directory and goes in it.
 
- ``RUN pip install --no-cache-dir -r requirements.txt``: Install the project dependencies using pip.

- ``COPY main.py .``: Copy the rest of the application code to the container which is `/app`

- ``CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "80"]``: Set the command to start the application.

## Usage

1. Clone the repository:

    ```sh
    git clone https://github.com/devopshobbies/docker-templates.git
    ```

2. Build the Docker image:

    ```sh
    cd docker-templates/13-FastApi
    docker build -t hello-fastapi .
    ```

3. Run the Docker container:

    ```sh
    docker container run --detach --publish 80:80 hello-fastapi
    ```
   > -d,--detach &emsp;&emsp;&emsp;&emsp; Run container in background and print container ID </br>
   > -p, --publish list &emsp;&emsp; Publish a container's port(s) to the host
