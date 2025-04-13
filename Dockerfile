# ---- Builder Stage ----
# Use a specific Rust version for reproducibility
FROM rust:1.79 as builder

# Set working directory
WORKDIR /app

# Copy manifests and lock file
COPY Cargo.toml Cargo.lock ./

# Build dependencies first to leverage Docker cache
# Create dummy src/main.rs and build.rs to satisfy cargo build
# This avoids copying the full source code just to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN touch build.rs
# Build only dependencies for the specific binary 'axum-folder'
# Adjust 'axum-folder' if your binary name in Cargo.toml is different
RUN cargo build --release --bin axum-folder
# Remove dummy files after dependency build
RUN rm -rf src build.rs

# Copy the actual source code, build script, and templates
COPY src ./src
COPY build.rs ./build.rs
COPY templates ./templates

# Build the application, this will use cached dependencies
# Ensure the binary name 'axum-folder' matches your Cargo.toml [package].name
RUN cargo build --release --bin axum-folder

# ---- Runtime Stage ----
# Use a minimal base image
FROM debian:bullseye-slim

# Set working directory
WORKDIR /app

# Create a non-root user and group for security
# Using fixed IDs for reproducibility
RUN groupadd --system --gid 1001 appuser && \
    useradd --system --uid 1001 --gid appuser appuser

# Copy the compiled binary from the builder stage
# Ensure the binary name 'axum-folder' matches your Cargo.toml [package].name
COPY --from=builder /app/target/release/axum-folder /app/axum-folder

# Copy static assets and templates required at runtime
COPY public ./public
COPY templates ./templates

# Ensure the binary is executable
RUN chmod +x /app/axum-folder

# Change ownership of the app directory to the non-root user
RUN chown -R appuser:appuser /app

# Switch to the non-root user
USER appuser

# Expose the default port the application listens on (from main.rs)
EXPOSE 3000

# Set the entrypoint command to run the application
# Ensure the binary name 'axum-folder' matches your Cargo.toml [package].name
CMD ["/app/axum-folder"]
