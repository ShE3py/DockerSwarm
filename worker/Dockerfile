FROM rust:1.83-alpine3.21 AS builder
RUN apk add --no-cache musl-dev

ENV RUSTUP_TERM_COLOR=always CARGO_TERM_COLOR=always

WORKDIR /checkout/
COPY Cargo.toml Cargo.lock ./
COPY worker worker

# Delete `hive` and `spy` lines from the workspace's members
RUN sed -i '3d;4d' Cargo.toml

ARG PROFILE
ENV PROFILE=${PROFILE:-release}

RUN cargo install --path worker --profile $PROFILE

FROM alpine:3.21 AS worker
COPY --from=builder /usr/local/cargo/bin/worker /usr/local/bin/worker
ENTRYPOINT ["worker"]
