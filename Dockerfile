FROM rust:1.83-alpine3.21 AS builder

ENV RUSTUP_TERM_COLOR=always CARGO_TERM_COLOR=always

RUN echo 'https://dl-cdn.alpinelinux.org/alpine/v3.21/community/' >> /etc/apk/repositories
RUN apk add --no-cache musl-dev trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/workspace
COPY . .

RUN cargo install --path worker
RUN cd hive && trunk build --release --minify --skip-version-check --color=always


FROM alpine:3.21 AS worker
COPY --from=builder /usr/local/cargo/bin/worker /usr/local/bin/worker
ENTRYPOINT ["worker"]


FROM nginx:alpine3.21-slim AS hive
COPY --from=builder /usr/src/workspace/hive/dist /usr/share/nginx/html
