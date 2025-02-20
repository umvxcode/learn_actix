# Use the official Rust image as the base image
FROM rust:1.84-slim AS builder

# Install pkg-config and OpenSSL development libraries
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies (this step caches them)
RUN cargo build --release

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a newer Debian base image for the final container
FROM debian:bookworm-slim

# Install necessary libraries for MySQL connectivity and OpenSSL
RUN apt-get update && apt-get install -y \
    default-libmysqlclient-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/latihan /usr/local/bin/latihan
RUN chmod +x /usr/local/bin/latihan

EXPOSE 8080

# Set the environment variable for the database connection
ENV DATABASE_URL=mysql://root:@host.docker.internal:3306/latihanrust

# Run the application
CMD ["latihan"]