![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Swift](https://img.shields.io/badge/swift-F54A2A?style=for-the-badge&logo=swift&logoColor=white)
# Learning Dockerfile for Beginners

This project contains a simple and practical example of a Dockerfile to help beginners learn how to use Docker.

## Prerequisites

- Docker installed on your system
- Basic knowledge of the Linux command line

## Here's what each line does

- ``FROM swift:latest as builder``: Use the official swift image as our base image.

- ``COPY app.swift .``: Copy the dependency file from the current directory to the container.

- ``RUN swiftc app.swift -o app``: Build the application.

- ``FROM swift:slim AS runner``: Base image for second stage

- ``COPY --from=builder /app .``: Copy the executable file from previous stage to directory

- ``CMD ["/app"]``: run the application

## Usage

1. Clone the repository:

```sh
git clone https://github.com/devopshobbies/docker-templates.git
```

2. Build the Docker image:

```sh
cd docker-templates/15-Swift
docker build -t hello-swift .
```

3. Run the Docker container:

```sh
docker container run hello-swift
```
