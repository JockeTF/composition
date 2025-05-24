FROM docker.io/library/alpine:latest AS build

RUN apk add ca-certificates transmission-daemon transmission-remote

ARG UID=51413
ENV TRANSMISSION_HOME="/app"
RUN adduser -D -u "$UID" -h /app app
WORKDIR /app
USER app

CMD ["/usr/bin/transmission-daemon", "--foreground"]
