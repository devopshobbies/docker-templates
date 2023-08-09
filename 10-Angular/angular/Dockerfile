
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
