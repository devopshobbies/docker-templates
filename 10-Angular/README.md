## Satge 1: create angular project

### 1-Install the Angular CLI globally using the following command:
```bash
npm install -g @angular/cli
```
### 2-Create a new Angular project using the Angular CLI by running the following command:
```bash
ng new my-angular-app
```
### 3-Change directory to the new project by running the following command:
```bash
cd my-angular-app
```

### Use with Docker Development Environments
### 4-Create a new file in the project root directory called Dockerfile and paste the following content into it:
```bash

# Set the syntax version for the Dockerfile
# syntax=docker/dockerfile:1.4
# Specify the base image to use for the build process and rename it as "builder"
# Install Node.js version 17.0.1 on the build platform specified by the environment variable $BUILDPLATFORM
FROM --platform=$BUILDPLATFORM node:17.0.1-bullseye-slim as builder

# Create a new directory called "project" and set it as the working directory for subsequent commands

WORKDIR /project
# Install the Angular CLI version 13 globally using npm
RUN npm install -g @angular/cli@13
# Copy the package.json and package-lock.json files into the "project" directory and install the required node modules using npm ci

COPY package.json package-lock.json ./
RUN npm ci
# Copy the remaining files in the current directory to the "project" directory in the image and set the default command to run when the container starts. The command is to serve the Angular app using the ng serve command with the host set to 0.0.0.0.

COPY . .
CMD ["ng", "serve", "--host", "0.0.0.0"]
# Start a new build stage using the "builder" image and rename it to "dev-envs"

FROM builder as dev-envs
# Install Git by updating the package index and then installing Git using the apt-get package manager using multi-line RUN instruction

RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF
# Create a new user called "vscode", create a new group called "docker", and add the "vscode" user to the "docker" group using multi-line RUN instruction

# Install Docker tools (cli, buildx, compose) from previously built image from the Docker Hub account named "gloursdocker/docker"
RUN <<EOF
useradd -s /bin/bash -m vscode
groupadd docker
usermod -aG docker vscode
EOF
# Set the default command to run when the container starts. The command is to serve the Angular app using the ng serve command with the host set to 0.0.0.0. This is the same command as the one in the first build stage.

COPY --from=gloursdocker/docker / /

CMD ["ng", "serve", "--host", "0.0.0.0"]

```

### Angular service

Project structure:
```
.
├── angular
│   ├── Dockerfile
│   ├── ...
│   ├── ...
│   ....
└── compose.yaml
```

[_compose.yaml_](compose.yaml)
```
services:
  web:
    build: angular
    ports:
    - 4200:4200
    ...

```
The compose file defines an application with one service `angular`. The image for the service is built with the Dockerfile inside the `angular` directory (build parameter).

When deploying the application, docker compose maps the container port 4200 to the same port on the host as specified in the file.
Make sure port 4200 is not being used by another container, otherwise the port should be changed.


## Deploy with docker compose

```
$ docker compose up -d
Creating network "angular_default" with the default driver
Building angular
Step 1/7 : FROM node:10
10: Pulling from library/node
c0c53f743a40: Pull complete
...
...
Successfully built efea5cef6851
Successfully tagged angular_web:latest
WARNING: Image for service web was built because it did not already exist. To rebuild this image you must use `docker-compose build` or `docker-compose up --build`.
Creating angular_web_1 ... done
```


## Expected result

Listing containers must show a container running and the port mapping as below:
```
$ docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                  NAMES
6884c228388e        angular_web         "docker-entrypoint.s…"   42 seconds ago      Up 36 seconds       0.0.0.0:4200->4200/tcp angular_web_1

```

After the application starts, navigate to `http://localhost:4200` in your web browser.

![page](output.png)

Stop and remove the container

```
$ docker compose down
```
