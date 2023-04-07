# Java (Spring boot) + Gradle Dockerfile template


## Important Note
The default path of generating jar file in Gradle is **/build/libs** directory of the project. However, in this project we changed it to **/target**. So, always make sure that the source path specified in the COPY command in the second stage of the Dockerfile matches the directory that is set in the build.gradle file.

In build.gradle :
```groovy
// generate app jar file in the /target directory of the project
bootJar {
    destinationDirectory = file("target")
}
```

In Dockerfile :

```dockerfile
# Copy generated jar file from /target directory to the working directory
COPY --from=build /app/target/Gradle-0.0.1-SNAPSHOT.jar .
```

## Run
Run following command to create docker image and run the container.

```sh
docker build -t java-gradle-dockerfile . && docker run -it -p 80:80 java-gradle-dockerfile
```