##################### Builder

FROM rust:1.68.2-alpine3.17 as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new app

# Copy the project to the destination in the builder
COPY . /usr/src/app

# Set the working directory
WORKDIR /usr/src/app

# This is the application build.
RUN cargo build --release

##################### Runtime

FROM scratch as runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/app/target/devopshobbies /usr/local/bin

USER 1000

# Run the application
ENTRYPOINT ["/usr/local/bin/devopshobbies"]
