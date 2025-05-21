FROM docker.io/library/rust:alpine AS build

RUN apk add build-base

ARG UID=29561
RUN adduser -D -u "$UID" -h /build build
WORKDIR /build
USER build

RUN --mount=dst="$CARGO_HOME/registry",id=TF7H3uKc,type=cache,uid="$UID" true \
 && cargo install --git https://github.com/JockeTF/rust-rpxy.git


FROM docker.io/library/alpine:latest

RUN apk add ca-certificates

COPY --from=build /usr/local/cargo/bin/rpxy .
COPY Config.toml .

ARG UID=29561
USER "$UID":"$UID"

CMD ["/rpxy", "-c", "Config.toml"]
