# Latest Rust image (as of 10.05.24) from Docker Hub as the base.
FROM rust:1.78 as builder

# Install necessary packages and libraries
RUN apt-get update && apt-get install -y openssl libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the current directory contents into the /app directory in the container.
COPY . /app

# Set the working directory inside the container to /app.
WORKDIR /app

# Compile the application statically
RUN cargo rustc --release -- -C target-feature=+crt-static

# Use a distroless static image for the runtime environment
FROM gcr.io/distroless/static

# Copy the compiled binary from the builder stage to this image.
COPY --from=builder /app/target/release/rust_stoic_quotes /app/

# Set the working directory inside the container to /app.
WORKDIR /app

# Expose the port that the application will listen on.
EXPOSE 8080

# Set the default command to run the application.
CMD ["./rust_stoic_quotes"]
