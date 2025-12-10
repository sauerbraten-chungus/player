FROM rust:1.88 as builder
WORKDIR /app

# Install protoc (Protocol Buffer compiler)
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# Copy build files
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY proto ./proto
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build with SQLx offline mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/player /app/
CMD ["./player"]
