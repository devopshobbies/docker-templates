FROM golang:1.18 as build 

WORKDIR /src

COPY go.mod .
# COPY go.sum .

RUN go mod download

COPY . .

RUN CGO_ENABLED=0 GO111MODULE=on go build -v -o sample -installsuffix cgo -ldflags="-w -s" ./cmd/*.go

FROM alpine

COPY --from=build /src/sample /

EXPOSE 80

RUN chmod +x /sample

CMD ["/sample"]
