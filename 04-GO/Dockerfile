# builder stage
# from golang alpine image
FROM golang:1.19-alpine as builder

# Installing Git for alpine
RUN apk add --no-cache git

# create work directory called src
WORKDIR /src

# build argument for framework
ARG FRAMEWORK=echo

# copy go.mod and go.sum
COPY $FRAMEWORK/go.mod $FRAMEWORK/go.sum ./

# download dependencies
RUN go mod download

# copy other files
COPY $FRAMEWORK $FRAMEWORK

# build golang app
RUN CGO_ENABLED=0 GO111MODULE=on go build -v -o /main -installsuffix cgo -ldflags="-w -s" ./$FRAMEWORK/*.go

# second stage
FROM alpine

# copy executable file
COPY --from=builder /main .

# allow main to be executed
RUN chmod +x /main

# expose port 80
EXPOSE 80

# run app executable file
CMD ["./main"]
