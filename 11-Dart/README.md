![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Dart](https://img.shields.io/badge/dart-%230175C2.svg?style=for-the-badge&logo=dart&logoColor=white)
# Learning Dockerfile for Beginners

This project contains a simple and practical example of a Dockerfile to help beginners learn how to use Docker.

## Prerequisites

- Docker installed on your system
- Basic knowledge of the Linux command line

## Here's what each line does

- ``FROM dart AS build``: Use the official dart image as our base image.

- ``WORKDIR /app``: Set the working directory in the container to /app.

- ``COPY pubspec.* /app/``: Copy the dependency file from the current directory to the container.

- ``RUN dart pub get --no-precompile``: Download and install the dependencies defined in pubspec file.

- ``COPY app.dart /app/``: Copy the application code to the container.

- ``RUN dart compile exe app.dart -o app``: Compiles the app.dart file into an executable binary called app.

- ``FROM debian:stable-slim``: Base image for second stage

- ``COPY --from=build /app/app /app/app``: Copy the executable binary from previous stage to directory

- ``CMD ["/app/app"]``: run the application

## Usage

1. Clone the repository:

```sh
git clone https://github.com/devopshobbies/docker-templates.git
```

2. Build the Docker image:

```sh
cd docker-templates/14-Dart
docker build -t hello-dart .
```

3. Run the Docker container:

```sh
docker container run hello-dart
```
