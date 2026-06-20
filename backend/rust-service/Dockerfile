FROM rust:1.90 as builder
WORKDIR /opt

RUN apt-get update && apt-get install -y \
    pkg-config \
    libvips-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release


FROM debian:bookworm-slim
WORKDIR /opt

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    libvips \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /opt/target/release/exppt .

EXPOSE 3000
CMD ["./chromatic"]