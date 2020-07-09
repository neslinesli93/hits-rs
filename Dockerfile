# Start with a rust alpine image
FROM clux/muslrust:1.44.1 as builder

WORKDIR /app
COPY Cargo.lock Cargo.toml /app/
COPY src /app/src/
RUN cargo build --release --jobs=4

# Copy the binary to an empty Docker image
FROM alpine
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/hits-rs /app/hits-rs
CMD ["./hits-rs"]
