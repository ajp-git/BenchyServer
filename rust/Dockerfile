# Étape 1 : Build
FROM rust:1.85-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Étape 2 : Image finale
FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/benchyserver-rust /
EXPOSE 8080
CMD ["/benchyserver-rust"]