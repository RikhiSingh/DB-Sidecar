# syntax=docker/dockerfile:1

FROM rust:1.78 as builder
WORKDIR /app
COPY . .
RUN cargo build -p sidecar --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/sidecar /usr/local/bin/sidecar
ENV RUST_LOG=info
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/sidecar", "serve"]


