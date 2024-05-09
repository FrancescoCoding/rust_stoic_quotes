# Use the official Rust image from Docker Hub as the base.
FROM rust

# Copy the current directory contents into the /app directory in the container.
COPY . /app

# Set the working directory inside the container to /app.
WORKDIR /app

# Compile the application in release mode.
RUN cargo build --release

# Set the default command to run the application.
CMD ["./target/release/rust_stoic_quotes"]
