# First stage: build the Java application
FROM maven:latest AS build
WORKDIR /app
COPY pom.xml .
RUN mvn dependency:go-offline
COPY src/ /app/src/
RUN mvn package

# Second stage: copy the built app from the first stage
FROM openjdk:11
WORKDIR /app
COPY --from=build /app/target/Maven-0.0.1-SNAPSHOT.jar /app/Maven-0.0.1-SNAPSHOT.jar
EXPOSE 80
CMD ["java", "-jar", "Maven-0.0.1-SNAPSHOT.jar"]

