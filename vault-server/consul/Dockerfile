FROM ubuntu:20.04

ENV CONSUL_VERSION=1.12.2
ENV HASHICORP_RELEASES=https://releases.hashicorp.com
ENV consulArch=amd64

RUN apt-get update && apt-get install -y unzip curl jq wget iputils-ping iproute2 && \
    rm -rf /var/lib/apt/lists/*

RUN useradd --system --home /etc/consul.d --shell /bin/false consul

RUN mkdir -p /etc/consul.d && \
    mkdir -p /opt/consul && \
    chown -R consul:consul /etc/consul.d && \
    chown -R consul:consul /opt/consul

RUN wget ${HASHICORP_RELEASES}/consul/${CONSUL_VERSION}/consul_${CONSUL_VERSION}_linux_${consulArch}.zip && \
    unzip -d /bin consul_${CONSUL_VERSION}_linux_${consulArch}.zip && \
    consul version

EXPOSE 8300 8301 8301/udp 8302 8302/udp 8500 8600 8600/udp
