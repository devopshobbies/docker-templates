##################### Builder

FROM rust:1.68.2-slim as builder

# Set the working directory
WORKDIR /usr/src/app

RUN apt-get update && apt-get install musl-tools --no-install-recommends -y

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# We want dependencies cached, so copy those first.
COPY Cargo.toml ./

# This is a dummy build to get the dependencies cached.
RUN mkdir src/ && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
  cargo build --target x86_64-unknown-linux-musl --release && \
  rm -f target/x86_64-unknown-linux-musl/release/deps/devopshobbies*

# Now copy in the rest of the sources
COPY . ./

# This is the application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

##################### Runtime

FROM alpine:3.17 as runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/devopshobbies /usr/local/bin

# Run the application
ENTRYPOINT ["/usr/local/bin/devopshobbies"]
