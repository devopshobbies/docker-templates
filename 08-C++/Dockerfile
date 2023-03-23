# Get the GCC preinstalled image from Docker Hub
FROM gcc:12.2.0 AS build

# Copy the current folder which contains C++ source code to the Docker image under /opt/test
COPY . /opt/test

# Specify the working directory
WORKDIR /opt/test

# Use GCC to compile the Test.cpp source file
RUN g++ -o app app.cpp

# Run the program output from the previous stage
FROM ubuntu:bionic
WORKDIR /opt/test
COPY --from=build /opt/test ./
CMD ["./app"]

