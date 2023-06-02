# Rust as the base image
FROM rust:1.69 as build

ENV APPNAME=sudobowl

# Create a new empty shell project
RUN USER=root cargo new --bin sudobowl
WORKDIR /sudobowl

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build
RUN rm ./target/release/deps/sudobowl*
RUN cargo build

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /sudobowl/target/debug/sudobowl /usr/app/sudobowl
# COPY --from=build /holodeck/target/release/holodeck/target/x86_64-unknown-linux-musl/release/holodeck .

EXPOSE 7878

# Run the binary
CMD ["/usr/app/sudobowl"]
