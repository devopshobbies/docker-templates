# NestJS Production docker file

This [Dockerfile](./Dockerfile) is used to build a clean and lightweight image of your NestJS application. please follow the instruction to understand what is happening clearly.

There is a NestJS example project that you can use and build the image after cloning the repository too.


## How to test and build?

First, make a clone from this repository using:

```bash
git clone https://github.com/devopshobbies/docker-templates.git
```

Build your image using the command:

```bash
docker build -t nestjs-temp docker-templates/01-JavaScript-Typescript-Backend/frameworks/nestjs
```

## Stage one, Build the NestJS application

### Identify the NodeJS alpine as a builder

```docker
FROM node:16-alpine as builder
```

### Set NODE_ENV to `build`

This line sets the NODE_ENV environment variable to build. This tells Node.js that we are in a build environment, which can be used to optimize the build process.

```docker
ENV NODE_ENV build
```

### Build commands

These lines set the user to node (a non-root user), create a working directory at /home/node, and copy `the package.json` and `package-lock.json` files to the working directory. The `npm ci` command installs the project's dependencies based on the `package-lock.json` file.

```docker
USER node
WORKDIR /home/node

COPY package\*.json ./
RUN npm ci

COPY --chown=node:node . .
RUN npm run build \
 && npm prune --production
```

Also, if you are using `yarn` package manager instead of `npm`, likely to change this part to:

```docker
USER node
WORKDIR /home/node

COPY package.json ./

# You have to copy the yarn.lock file
COPY yarn.lock ./
RUN yarn install --immutable --immutable-cache --check-cache

COPY --chown=node:node . .
RUN yarn build \
  && yarn install --production --ignore-scripts --prefer-offline
```

## Stage two, create the lightweight image

### set the base image to node:16-alpine

```docker
FROM node:16-alpine
```

### Set NODE_ENV to `build`

```docker
ENV NODE_ENV production
```

### Copy the necessary files.

These lines copy the `package.json` and `package-lock.json` files, the node_modules directory, and the `dist` directory from the builder stage to the current stage. The `--from=builder` flag specifies that the files should be copied from the builder stage, and the `--chown=node:node` flag sets the ownership of the copied files to the node user.

```docker
USER node
WORKDIR /home/node

COPY --from=builder --chown=node:node /home/node/package\*.json ./
COPY --from=builder --chown=node:node /home/node/node_modules/ ./node_modules/
COPY --from=builder --chown=node:node /home/node/dist/ ./dist/
```

If you are using `yarn`, add this extra line to copy the `yarn.lock` file.
```docker
COPY --from=builder --chown=node:node /home/node/yarn.lock ./
```

## The final step, add the startup command

```docker
CMD ["node", "dist/main.js"]
```

