# Use an official Flutter runtime as a parent image
FROM cirrusci/flutter:2.8.1 AS builder

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Run the following commands inside the container
RUN flutter pub get
RUN flutter build web --release

# Use an official Nginx runtime as a parent image
FROM nginx:1.21-alpine

# Copy the built Flutter web files into the Nginx web server directory
COPY --from=builder /app/build/web /usr/share/nginx/html

# Expose port 80 for the Nginx web server
EXPOSE 80

# Start the Nginx web server when the container starts
CMD ["nginx", "-g", "daemon off;"]
