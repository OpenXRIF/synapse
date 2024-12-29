# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the entire crate directory
COPY . .

# Build the application
RUN cargo build --release

# Set the environment variable for info level logging
ENV RUST_LOG=info

# Run the application
CMD ["cargo", "run", "--release"]
