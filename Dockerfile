# Start with a rust alpine image
FROM clux/muslrust:1.44.1 as builder

# Build a fake app, in order to cache the cargo deps layer
WORKDIR /app
RUN USER=root cargo new hits-rs
WORKDIR /app/hits-rs
COPY Cargo.lock Cargo.toml ./
RUN cargo build --release

# Build the actual app
COPY src ./src
RUN cargo install --force --target x86_64-unknown-linux-musl --path . --root /usr/local

# Copy the binary to an empty Docker image
FROM alpine:3.12
WORKDIR /app
COPY --from=builder /usr/local/bin/hits-rs /app/hits-rs
CMD ["./hits-rs"]
