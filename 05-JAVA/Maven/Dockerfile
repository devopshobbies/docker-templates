# First stage: build the Java application
FROM maven:3.8.1-jdk-11-slim AS build
WORKDIR /app
COPY pom.xml .
RUN mvn dependency:go-offline
COPY src/ src/
RUN mvn package

# Second stage: copy the built app from the first stage
FROM openjdk:11-slim
WORKDIR /app
COPY --from=build /app/target/Maven-0.0.1-SNAPSHOT.jar .
EXPOSE 80
CMD ["java", "-jar", "Maven-0.0.1-SNAPSHOT.jar"]

