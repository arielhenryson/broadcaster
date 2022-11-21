# 1. This tells docker to use the Rust official image
FROM rust:1.65.0-slim AS builder

# 2. Copy the files in your machine to the Docker image
COPY ./ ./app

# set the work directory
WORKDIR /app

# Build your program for release
RUN cargo build --release

# use google distroless as runtime image
FROM gcr.io/distroless/cc-debian11

# copy app from builder stage
COPY --from=builder /app/target/release/broadcaster /app/broadcaster

# set the work directory
WORKDIR /app

# Run the binary
CMD ["./broadcaster"]