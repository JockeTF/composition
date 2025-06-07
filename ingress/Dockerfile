FROM docker.io/library/rust:alpine AS build

RUN apk add build-base cmake

ARG UID=29561
RUN adduser -D -u "$UID" -h /build build
WORKDIR /build
USER build

ARG CPU='ivybridge'
ENV RUSTFLAGS="-C target-cpu=$CPU"

COPY --chown="$UID" Cargo.toml .
RUN --mount=dst="$CARGO_HOME/registry",id=TF7H3uKc,type=cache,uid="$UID" true \
 && mkdir src \
 && touch src/lib.rs \
 && cargo build --release

COPY --chown="$UID" . .
RUN --mount=dst="$CARGO_HOME/registry",id=TF7H3uKc,type=cache,uid="$UID" true \
 && cargo build --release


FROM scratch

COPY --from=build /build/target/release/ingress .

ARG UID=29561
USER "$UID":"$UID"

CMD ["/ingress"]
