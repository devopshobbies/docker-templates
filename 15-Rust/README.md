# Rust Dockerfile template

## Introduction

This docker templates demonstrate how to use `Dockerfile` for your Rust projects.

## usage

```bash
# build the image
docker build -t my-rust-app .

# run the app
docker run --rm --name devopshobbies my-rust-app 8080

# successful curl to the application
curl 127.0.0.1:8080 -i

# failing curl to the application
curl 127.0.0.1:8080/something/else -i
```
