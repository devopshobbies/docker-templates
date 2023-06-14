# Stage 1: Build the Haskell program
FROM haskell:slim AS builder

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Compile the Haskell program
RUN ghc -o main app.hs

# Stage 2: Create the final image
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /app/main /app/main

# Run the program when the container starts
CMD ["/app/main"]
