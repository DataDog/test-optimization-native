# Use the official Rust image as the base
FROM rust:1.85-slim-bookworm

RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Set the working directory
WORKDIR /usr/src/test-optimization-sdk

# Copy the manifest files first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src
COPY build.rs ./

ENV DD_TRACE_DEBUG=1

# Run the tests
CMD ["cargo", "test", "--", "--nocapture"] 