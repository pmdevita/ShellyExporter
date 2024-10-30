FROM rust:1.82-alpine3.20 as builder
WORKDIR /usr/src/myapp
RUN apk add alpine-sdk openssl-dev openssl musl-dev
COPY . .
RUN cargo install --path .

FROM alpine:3.20
RUN apk add openssl-dev
COPY --from=builder /usr/local/cargo/bin/shellyexporter /usr/local/bin/shellyexporter
CMD ["shellyexporter"]
