# Optimized multi-stage Dockerfile for Rust Actix-web
# Based on cargo-chef for optimal layer caching

####################################################################################################
## Cargo Chef - Dependency Planner
####################################################################################################
FROM rustlang/rust:nightly-bookworm AS chef

# Install cargo-chef for dependency caching
RUN cargo install cargo-chef --locked

WORKDIR /app

####################################################################################################
## Plan Dependencies
####################################################################################################
FROM chef AS planner

# Copy manifest files and source structure for dependency analysis
COPY Cargo.toml Cargo.lock ./
COPY entity/ ./entity/
COPY migration/ ./migration/
COPY src/ ./src/

# Generate recipe for dependencies
RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Build Dependencies
####################################################################################################
FROM chef AS dependencies

# Copy the recipe and workspace structure
COPY --from=planner /app/recipe.json recipe.json
COPY --from=planner /app/Cargo.toml /app/Cargo.lock ./
COPY --from=planner /app/entity/ ./entity/
COPY --from=planner /app/migration/ ./migration/

# Build dependencies only
RUN cargo chef cook --release --recipe-path recipe.json

####################################################################################################
## Builder
####################################################################################################
FROM dependencies AS builder

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

####################################################################################################
## Runtime Environment
####################################################################################################
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies and CA certificates
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd -r appuser && \
    useradd -r -g appuser -s /bin/false -M appuser

# Set working directory
WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/actix_rust_restful ./

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port (configurable via environment)
EXPOSE 3000

# Run the application
CMD ["./actix_rust_restful"]
