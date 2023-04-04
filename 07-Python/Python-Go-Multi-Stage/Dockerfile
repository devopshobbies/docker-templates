## Golang image for some modules ##
FROM golang:1.18.1-bullseye as go-build

# Install go binary package
RUN go install github.com/signorrayan/helloworld-go@latest && \
    cp $GOPATH/bin/* /usr/local/bin/


## Pull official base image ##
FROM python:3.10-slim-bullseye

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1
ENV PATH="/usr/src/devops_hobbies/webapp/scripts:${PATH}"

WORKDIR /usr/src/devops_hobbies/

RUN adduser --disabled-password --gecos '' devops

COPY . .

RUN apt-get update && \
    apt-get -y install --no-install-recommends gcc musl-dev netcat
    # && rm -rf /var/lib/apt/lists/*

RUN pip install --upgrade pip && \
    pip install -r ./webapp/requirements.txt

RUN chown -R devops:devops /usr/src/devops_hobbies

USER devops

RUN sed -i 's/\r$//g' /usr/src/devops_hobbies/entrypoint.sh && \
    chmod +x /usr/src/devops_hobbies/entrypoint.sh

# Copy from the bo-build stage
COPY --from=go-build /usr/local/bin/helloworld-go /usr/src/devops_hobbies/webapp/scripts/

ENTRYPOINT ["/usr/src/devops_hobbies/entrypoint.sh"]