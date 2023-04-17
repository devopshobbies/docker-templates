# Use the official Haskell image as a parent image
FROM haskell:latest

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Compile the Haskell program
RUN ghc -o main app.hs

# Run the program when the container starts
CMD ["./main"]
