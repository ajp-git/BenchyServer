FROM rust:1.85-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN apt-get update && apt-get install -y musl-tools && rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/benchyserver-rust /
EXPOSE 8080
CMD ["/benchyserver-rust"]