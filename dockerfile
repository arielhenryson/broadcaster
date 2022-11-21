# stage 1 generate a recipe file for dependencies
FROM rust:1.65.0-slim as planner

# set the work directory
WORKDIR /app

# Install cargo chef
RUN cargo install cargo-chef

# copy the source code
COPY . .

# create the recipe.json
RUN cargo chef prepare --recipe-path recipe.json

# stage 2 build our dependencies
FROM rust:1.65.0-slim as cacher

# set the work directory
WORKDIR /app

# Install cargo chef
RUN cargo install cargo-chef

# copy the recipe from the planner stage
COPY --from=planner /app/recipe.json recipe.json

# compile the dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# This tells docker to use the Rust official image
FROM rust:1.65.0-slim AS builder

# Copy the files in your machine to the Docker image
COPY ./ ./app

# set the work directory
WORKDIR /app

# copy dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

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