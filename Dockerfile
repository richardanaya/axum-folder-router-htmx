# ---- Builder Stage ----
# Use a specific Rust version for consistency
FROM rust:1.79 as builder

# Set the working directory in the container
WORKDIR /app

# Copy the manifests, lock file, source code, build script, and templates
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY build.rs ./build.rs
COPY templates ./templates

# Build the application binary in release mode
# Ensure the binary name 'axum-folder' matches your Cargo.toml [package].name
RUN cargo build --release --bin axum-folder

# ---- Runtime Stage ----
# Use a minimal Debian image for the final stage
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
# Ensure the binary name 'axum-folder' matches your Cargo.toml [package].name
COPY --from=builder /app/target/release/axum-folder /app/axum-folder

# Copy the public assets folder required by the application
COPY public ./public

# Copy the templates folder required by Askama at runtime
COPY templates ./templates

# Set the default port the application will listen on.
# Your main.rs reads this environment variable.
# This can be overridden when running the container (e.g., docker run -p 8080:8080 -e PORT=8080 ...)
ENV PORT=3000

# Expose the port specified by the PORT environment variable.
# Docker uses this information for networking.
EXPOSE $PORT

# Command to run the application when the container starts
# Ensure the binary name 'axum-folder' matches your Cargo.toml [package].name
CMD ["/app/axum-folder"]
