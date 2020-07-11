# Start with a rust alpine image
FROM clux/muslrust:1.44.1 as builder

# Build only dependencies, to cache this layer
WORKDIR /app
COPY Cargo.lock Cargo.toml /app/
RUN mkdir /app/src && echo "fn main() {println!(\"cargo:rerun-if-changed=\\\"/app/src/main.rs\\\"\");}" >> /app/src/main.rs
RUN cargo build --release --jobs=4

# Build actual app
COPY src /app/src/
RUN cargo build --release --jobs=4

# Copy the binary to an empty Docker image
FROM alpine:3.12
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/hits-rs /app/hits-rs
CMD ["./hits-rs"]
