FROM rust:1-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release -p sdkwork-api-mahjong-standalone-gateway

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /src/target/release/sdkwork-api-mahjong-standalone-gateway /usr/local/bin/sdkwork-api-mahjong-standalone-gateway
ENV SDKWORK_MAHJONG_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:8097
ENV MAHJONG_API_BIND=0.0.0.0:8097
EXPOSE 8097
CMD ["sdkwork-api-mahjong-standalone-gateway"]
