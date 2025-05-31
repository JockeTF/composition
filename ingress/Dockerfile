FROM docker.io/library/rust:alpine AS build

RUN apk add build-base ca-certificates-bundle

ARG UID=29561
RUN adduser -D -u "$UID" -h /build build
WORKDIR /build
USER build

RUN --mount=dst="$CARGO_HOME/registry",id=TF7H3uKc,type=cache,uid="$UID" true \
 && cargo install --git https://github.com/junkurihara/rust-rpxy.git


FROM scratch

COPY --from=build /etc/ssl /etc/ssl
COPY --from=build /usr/local/cargo/bin/rpxy .
COPY Config.toml .

ARG UID=29561
USER "$UID":"$UID"

CMD ["/rpxy", "-c", "Config.toml"]
