# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:latest as builder

# Install Cross Compiler
# RUN apt-get update
# RUN apt-get install musl-tools -y
# RUN rustup target add x86_64-unknown-linux-musl

# Setup working directory
WORKDIR /usr/src/backend
COPY . .

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo install --target=x86_64-unknown-linux-musl --path .

# Build application
RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend

RUN apt-get update
RUN apt-get install -y libpq-dev

EXPOSE 8080

CMD backend

