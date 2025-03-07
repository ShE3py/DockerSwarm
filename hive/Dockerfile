FROM rust:1.83-alpine3.21 AS builder
RUN apk add --no-cache musl-dev

ENV RUSTUP_TERM_COLOR=always CARGO_TERM_COLOR=always

RUN echo 'https://dl-cdn.alpinelinux.org/alpine/v3.21/community/' >> /etc/apk/repositories
RUN apk add --no-cache trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /checkout/
COPY Cargo.toml Cargo.lock ./
COPY hive hive

# Delete `spy` and `worker` lines from the workspace's members
RUN sed -i '4d;5d' Cargo.toml

ARG PROFILE
ENV PROFILE=${PROFILE:-release}

RUN cd hive && trunk build --cargo-profile ${PROFILE} --minify --skip-version-check --color=always

FROM nginx:alpine3.21-slim AS hive
COPY --from=builder /checkout/hive/dist /usr/share/nginx/html
