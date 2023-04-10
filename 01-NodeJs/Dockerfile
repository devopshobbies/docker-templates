FROM node:fermium-alpine AS environment


ARG MS_HOME=/app
ENV MS_HOME="${MS_HOME}"

ENV MS_SCRIPTS="${MS_HOME}/scripts"

ENV USER_NAME=node USER_UID=1000 GROUP_NAME=node GROUP_UID=1000

WORKDIR "${MS_HOME}"

# Build
FROM environment AS develop

COPY ["./package.json", "./package-lock.json", "${MS_HOME}/"]

FROM develop AS builder
COPY . "${MS_HOME}"


RUN PATH="$(npm bin)":${PATH} \
  && npm ci \
  && npm run test:ci \
  && npm run test:e2e \
  && npm run-script build \
  # Clean up dependencies for production image
  && npm install --frozen-lockfile  --production && npm cache clean --force

# Serve
FROM environment AS prod


COPY ["./scripts/docker-entrypoint.sh", "/usr/local/bin/entrypoint"]
COPY ["./scripts/bootstrap.sh", "/usr/local/bin/bootstrap"]
COPY --from=builder "${MS_HOME}/node_modules" "${MS_HOME}/node_modules"
COPY --from=builder "${MS_HOME}/dist" "${MS_HOME}/dist"

RUN  \
  apk --update add --no-cache tini bash \
  && deluser --remove-home node \
  && addgroup -g ${GROUP_UID} -S ${GROUP_NAME} \
  && adduser -D -S -s /sbin/nologin -u ${USER_UID} -G ${GROUP_NAME} "${USER_NAME}" \
  && chown -R "${USER_NAME}:${GROUP_NAME}" "${MS_HOME}/" \
  && chmod a+x \
    "/usr/local/bin/entrypoint" \
    "/usr/local/bin/bootstrap" \
  && rm -rf \    
    "/usr/local/lib/node_modules" \
    "/usr/local/bin/npm" \
    "/usr/local/bin/docker-entrypoint.sh"
USER "${USER_NAME}"


EXPOSE 8085

ENTRYPOINT [ "/sbin/tini", "--", "/usr/local/bin/entrypoint" ]
