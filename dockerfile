# Stage 1 generate a recipe file for dependencies
FROM rust:1.65.0-slim as planner

# Set the work directory
WORKDIR /app

# Install cargo chef
RUN cargo install cargo-chef

# Copy the source code
COPY . .

# Create the recipe.json
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2 build our dependencies
FROM rust:1.65.0-slim as cacher

# Set the work directory
WORKDIR /app

# Install cargo chef
RUN cargo install cargo-chef

# Copy the recipe from the planner stage
COPY --from=planner /app/recipe.json recipe.json

# Compile the dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# This tells docker to use the Rust official image
FROM rust:1.65.0-slim AS builder

# CREATE appuser
ENV USER=web
ENV UID=1001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/noneexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Copy the files in your machine to the Docker image
COPY ./ ./app

# Set the work directory
WORKDIR /app

# Copy dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Build your program for release
RUN cargo build --release

# Use google distroless as runtime image
FROM gcr.io/distroless/cc-debian11

# Import from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy app from builder stage
COPY --from=builder /app/target/release/broadcaster /app/broadcaster

# Set the work directory
WORKDIR /app

USER web:web

# Run the binary
CMD ["./broadcaster"]