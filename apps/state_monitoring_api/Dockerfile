
FROM rust:1.90 AS builder

WORKDIR /usr/src/app

# Copy only Cargo.toml and Cargo.lock first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs

# Build dependencies
RUN cargo build --release

# Now copy the actual source code
COPY src src

# Touch main.rs to force rebuild
RUN touch src/main.rs

# Build the actual application
RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/statemon /usr/local/bin/statemon

ENV RUST_LOG="statemon=info,actix_web=info"

CMD ["statemon"]