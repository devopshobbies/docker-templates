# Go Dockerfile template

```Dockerfile``` template for ```Golang``` applications.

# Just run !

```sh

docker build -t go-dockerfile . && docker run -it -p 80:80 go-dockerfile

```

## Choose Framework

By default, the selected framework is ```echo```. To select other frameworks, just pass
the framework name in ```docker build argument```.

```sh

docker build --build-arg FRAMEWORK=fiber -t go-dockerfile . && docker run -it -p 80:80 go-dockerfile

```

## Supported Frameworks

- ```echo```
- ```fiber```
