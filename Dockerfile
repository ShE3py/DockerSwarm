FROM rust:1.83-alpine3.21 AS builder

WORKDIR /usr/src/workspace
ENV RUSTUP_TERM_COLOR=always CARGO_TERM_COLOR=always

COPY . .

RUN echo 'https://dl-cdn.alpinelinux.org/alpine/v3.21/community/' >> /etc/apk/repositories
RUN apk add --no-cache musl-dev trunk
RUN RUSTUP_TERM_COLOR=always rustup target add wasm32-unknown-unknown

RUN cargo install --path worker
RUN cd hive && trunk build --release --minify --skip-version-check --color=always


FROM alpine:3.21 AS worker
COPY --from=builder /usr/local/cargo/bin/worker /usr/local/bin/worker
CMD ["worker"]


FROM nginx:alpine3.21-slim AS hive
COPY --from=builder /usr/src/workspace/hive/dist /usr/share/nginx/html
