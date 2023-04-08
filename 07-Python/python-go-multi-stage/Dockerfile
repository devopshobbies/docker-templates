## Golang image for some modules ##
FROM golang:1.18.1-bullseye as go-build

# Install go binary package
RUN go install github.com/signorrayan/helloworld-go@latest && \
    cp $GOPATH/bin/* /usr/local/bin/


## Pull official base image ##
FROM python:3.10-slim-bullseye as main-build

ENV PATH="/usr/src/devops_hobbies/webapp/scripts:${PATH}"

WORKDIR /usr/src/devops_hobbies/

COPY . .

RUN apt-get update && \
    apt-get -y install --no-install-recommends gcc musl-dev netcat && \
    pip install --upgrade pip && \
    pip install -r ./webapp/requirements.txt &&  \
    adduser --disabled-password --gecos '' devops && \
    chown -R devops:devops /usr/src/devops_hobbies && \
    sed -i 's/\r$//g' /usr/src/devops_hobbies/entrypoint.sh && \
    chmod +x /usr/src/devops_hobbies/entrypoint.sh
    # && rm -rf /var/lib/apt/lists/*

USER devops

# Copy from the go-build stage
COPY --from=go-build /usr/local/bin/helloworld-go /usr/src/devops_hobbies/webapp/scripts/

ENTRYPOINT ["/usr/src/devops_hobbies/entrypoint.sh"]