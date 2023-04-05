# Java + Maven Dockerfile template

Run following command to create docker image and run the container.

```sh
docker build -t java-maven-dockerfile . && docker run -it -p 80:80 java-maven-dockerfile
```