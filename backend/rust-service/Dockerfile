FROM rust:1.90 as builder
WORKDIR /opt

RUN apt-get update && apt-get install -y \
    pkg-config \
    libvips-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release


FROM debian:trixie-slim
WORKDIR /opt

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    libvips \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /opt/target/release/chromatic .

EXPOSE 3000
HEALTHCHECK --interval=2s --timeout=3s --retries=10 --start-period=5s \
    CMD nc -z localhost 3000 || exit 1
CMD ["./chromatic"]