# Rust Dockerfile template

## Introduction

This docker templates demonstrate how to use `Dockerfile` for your Rust projects.

## usage

```bash
# build the image
docker build -t my-rust-app .

# run the app
docker run --rm --name rusty my-rust-app

# run with ENVs
docker run --rm -e RUST_DOCKER_NAME="Foo" --name rusty my-rust-app
```
