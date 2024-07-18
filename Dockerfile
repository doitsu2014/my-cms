# Use the official Rust image as the base image
FROM rust:1.79 as builder

# Set the working directory inside the container
WORKDIR /usr/src/my-cms

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Create a new stage from scratch
FROM rust:1.79-alpine

COPY --from=builder /usr/src/my-cms/target/release /usr/local/bin/

# Run the application
CMD ["/usr/local/bin/release/main"]
