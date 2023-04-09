FROM alpine:3.14 AS preInstaller
RUN mkdir /etc/vault.d
RUN apk --no-cache add wget

FROM preInstaller
ARG VAULT_VERSION=1.12.2
RUN wget --quiet --output-document=/tmp/vault.zip https://releases.hashicorp.com/vault/${VAULT_VERSION}/vault_${VAULT_VERSION}_linux_amd64.zip && \
    unzip /tmp/vault.zip -d /bin && \
    rm -f /tmp/vault.zip
    
EXPOSE 8200 8201
