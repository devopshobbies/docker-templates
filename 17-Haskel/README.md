![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Haskell](https://img.shields.io/badge/Haskell-5e5086?style=for-the-badge&logo=haskell&logoColor=white)
# Learning Dockerfile for Beginners

This project contains a simple and practical example of a Dockerfile to help beginners learn how to use Docker.

## Prerequisites

- Docker installed on your system
- Basic knowledge of the Linux command line

## Here's what each line does

- ``FROM haskel:slim as builder``: Use the official haskel image as our base image.

- ``WORKDIR /app``: Set the working directory to /app

- ``COPY app.hs .``: Copy the current directory contents into the container at /app.

- ``RUN ghc -o main app.hs``: Compile the Haskell program.

- ``FROM debian:buster-slim``: Base image for second stage.

- ``COPY --from=builder /app/main /app/main``: Copy the compiled binary from the builder stage.

- ``CMD ["/app/main"]``: run the application

## Usage

1. Clone the repository:

```sh
git clone https://github.com/devopshobbies/docker-templates.git
```

2. Build the Docker image:

```sh
cd docker-templates/17-Haskel
docker build -t hello-haskel .
```

3. Run the Docker container:

```sh
docker container run hello-haskel
```
