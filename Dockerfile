FROM rust:1.82-alpine3.20 as builder
WORKDIR /app
RUN apk add alpine-sdk musl-dev
COPY . .
RUN cargo build --release

FROM alpine:3.20
COPY --from=builder /app/target/release/ShellyExporter /usr/local/bin/
CMD ["ShellyExporter"]
