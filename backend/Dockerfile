# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:latest as builder

# Setup working directory
WORKDIR /usr/src/backend
COPY . .

# Build application
RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend

RUN apt update
RUN apt install -y libpq-dev

EXPOSE 8080

CMD backend

