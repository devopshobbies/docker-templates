# build stage
# from alpine image
FROM golang:1.19-alpine as builder

# create work directory called src
WORKDIR /src

# copy go.mod and go.sum
COPY go.mod go.sum ./

# download dependencies
RUN go mod download

# copy other files
COPY . .

# build golang app
RUN CGO_ENABLED=0 GO111MODULE=on go build -v -o /main -installsuffix cgo -ldflags="-w -s" ./cmd/*.go

# second stage
FROM alpine

# copy executable file
COPY --from=builder /src/main /main

# expose port 80
EXPOSE 80

# run app executeable file
CMD ["/main"]