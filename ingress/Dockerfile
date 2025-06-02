FROM docker.io/library/rust:alpine AS build

RUN apk add build-base ca-certificates-bundle

ARG UID=29561
RUN adduser -D -u "$UID" -h /build build
WORKDIR /build
USER build

ARG FEATURES='acme,rustls-backend'
ARG SOURCE='https://github.com/junkurihara/rust-rpxy.git'
RUN --mount=dst="$CARGO_HOME/registry",id=TF7H3uKc,type=cache,uid="$UID" true \
 && cargo install --no-default-features --features="$FEATURES" --git="$SOURCE"


FROM scratch

COPY --from=build /etc/ssl /etc/ssl
COPY --from=build /usr/local/cargo/bin/rpxy .
COPY Config.toml .

ARG UID=29561
USER "$UID":"$UID"

CMD ["/rpxy", "-c", "Config.toml"]
