# Build stage
FROM rust:bookworm as builder

WORKDIR /usr/src/app

# Install cmake which is required for compiling boring-sys2/rquest
RUN apt-get update && apt-get install -y cmake clang

COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary CA certificates for rquest to make HTTPS requests
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/SoSearch /app/SoSearch

ENV PORT=10080
EXPOSE 10080

CMD ["./SoSearch"]
