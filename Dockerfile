# ---- Builder Stage ----
# Use a specific Rust version for consistency
FROM rust:1.83 as builder

# Set the working directory in the container
WORKDIR /app

# copy everything so we can build

COPY . .


# Build the application binary in release mode
# This will reuse the cached dependency layer
# Ensure the binary name 'axum-folder' matches your Cargo.toml [package].name
RUN cargo build --release --bin axum-folder

# Set the default port the application will listen on.
# Your main.rs reads this environment variable.
# This can be overridden when running the container (e.g., docker run -p 8080:8080 -e PORT=8080 ...)
ENV PORT=3000

# Expose the port specified by the PORT environment variable.
# Docker uses this information for networking.
EXPOSE $PORT

# Command to run the application when the container starts
# Ensure the binary name 'axum-folder' matches your Cargo.toml [package].name
CMD ["/app/target/release/axum-folder"]
