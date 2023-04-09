FROM alpine:3.14 AS preInstaller
RUN mkdir -p /etc/consul.d && \
    mkdir -p /opt/consul
RUN apk --no-cache add wget

FROM preInstaller
ARG CONSUL_VERSION=1.12.2
RUN wget --quiet --output-document=/tmp/consul.zip https://releases.hashicorp.com/consul/${CONSUL_VERSION}/consul_${CONSUL_VERSION}_linux_amd64.zip && \
    unzip /tmp/consul.zip -d /bin && \
    rm -f /tmp/consul.zip

EXPOSE 8500
