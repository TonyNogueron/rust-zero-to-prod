# Base Image
FROM rust:latest AS builder

# Change the working directory
WORKDIR /app
# Install the required dependencies
RUN apt update && apt install lld clang -y

# Copy the source code to the working directory
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder to our runtime
COPY --from=builder /app/target/release/zero_to_prod_example zero_to_prod_example
# We need config file at runtime
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT [ "./zero_to_prod_example" ]
