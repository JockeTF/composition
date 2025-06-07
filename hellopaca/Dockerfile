FROM docker.io/library/rust:alpine AS build

RUN apk add build-base

ARG UID=49211
RUN adduser -D -u "$UID" -h /build build
WORKDIR /build
USER build

COPY --chown="$UID" Cargo.toml .
RUN --mount=dst="$CARGO_HOME/registry",id=VcK9VNqc,type=cache,uid="$UID" true \
 && mkdir src \
 && touch src/lib.rs \
 && cargo build --release

COPY --chown="$UID" . .
RUN --mount=dst="$CARGO_HOME/registry",id=VcK9VNqc,type=cache,uid="$UID" true \
 && cargo build --release


FROM scratch

COPY --from=build /build/target/release/hellopaca .

ARG UID=49211
ENV BIND="[::]:$UID"
USER "$UID":"$UID"

CMD ["/hellopaca"]
