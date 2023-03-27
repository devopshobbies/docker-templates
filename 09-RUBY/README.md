![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Rails](https://img.shields.io/badge/rails-%23CC0000.svg?style=for-the-badge&logo=ruby-on-rails&logoColor=white)
# Learning Dockerfile for Beginners

This project contains a simple and practical example of a Dockerfile to help beginners learn how to use Docker.

## Prerequisites

- Docker installed on your system
- Basic knowledge of the Linux command line

## Here's what each line does

- ``FROM ruby:2.7-alpine``: Use the official Ruby 2.7 image as our base image. We're using the Alpine version to keep the image small.

- ``WORKDIR /app``: Set the working directory in the container to /app.

- ``COPY Gemfile Gemfile.lock ./``: Copy the Gemfile and Gemfile.lock files from the current directory to the container.

- ``RUN bundle install``: Install the project dependencies using Bundler.

- ``COPY . .``: Copy the rest of the application code to the container.

- ``CMD ["ruby", "app.rb"]``: Set the command to start the application.

## Usage

1. Clone the repository:

```sh
git clone https://github.com/devopshobbies/docker-templates.git
```

2. Build the Docker image:

```sh
cd docker-templates
docker build -t hello-ruby .
```

3. Run the Docker container:

```sh
docker run -it --rm hello-ruby
```
