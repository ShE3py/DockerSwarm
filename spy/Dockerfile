FROM rust:1.83-alpine3.21 AS builder
RUN apk add --no-cache musl-dev

ENV RUSTUP_TERM_COLOR=always CARGO_TERM_COLOR=always

WORKDIR /checkout/
COPY Cargo.toml Cargo.lock ./
COPY spy spy

# FIXME: hack
COPY worker/src/websocket.rs worker/src/websocket.rs

# Delete `hive` and `worker` lines from the workspace's members
RUN sed -i '3d;5d' Cargo.toml

ARG PROFILE
ENV PROFILE=${PROFILE:-release}

RUN cargo install --path spy --profile $PROFILE

FROM alpine:3.21 AS spy
RUN apk add --no-cache docker

COPY --from=builder /usr/local/cargo/bin/spy /usr/local/bin/spy
ENTRYPOINT ["spy"]
