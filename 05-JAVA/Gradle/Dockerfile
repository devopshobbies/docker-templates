# First stage: build the Java application using Gradle
FROM gradle:7.3.3-jdk11-alpine AS build
WORKDIR /app
COPY . .
RUN ./gradlew bootJar -DskipTests

# Second stage: copy the built app from the first stage and run it
FROM openjdk:11-slim
WORKDIR /app
COPY --from=build /app/target/Gradle-0.0.1-SNAPSHOT.jar .
CMD "java" "-jar" "Gradle-0.0.1-SNAPSHOT.jar"
