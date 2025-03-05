FROM rust:1.83-alpine3.21 AS builder
RUN apk add --no-cache musl-dev

ENV RUSTUP_TERM_COLOR=always CARGO_TERM_COLOR=always

WORKDIR /checkout/
COPY . .

RUN cargo install --path spy --profile dev

FROM alpine:3.21 AS worker
COPY --from=builder /usr/local/cargo/bin/spy /usr/local/bin/spy
ENTRYPOINT ["spy"]
