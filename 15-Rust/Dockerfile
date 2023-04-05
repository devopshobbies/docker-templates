##################### Builder

FROM rust:1.68.2-alpine3.17 as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new app

# Copy the project to the destination in the builder
COPY . /usr/src/app

# Set the working directory
WORKDIR /usr/src/app

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

##################### Runtime

FROM alpine:3.17 as runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin

# Run the application
ENTRYPOINT ["/usr/local/bin/app"]
