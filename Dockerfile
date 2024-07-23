# # Use the official Rust image as the base image
# FROM rust:1.79 as builder
#
# # Set the working directory inside the container
# WORKDIR /usr/src/my-cms
#
# # Copy the source code
# COPY . .
#
# # Build the application in release mode
# RUN mkdir -p /usr/local/bin/ & cargo install --path . --target-dir /usr/local/bin/
#
# # Create a new stage from scratch
# FROM rust:1.79-alpine as runtime
#
# COPY --from=builder /usr/local/bin/release /usr/local/bin
#
# # Run the application
# CMD ["/usr/local/bin/my-cms-api"]

FROM rust:1.79 as build

WORKDIR /usr/local/my-cms

COPY . .

RUN cargo build --release


FROM rust:1.79-slim
WORKDIR /app
COPY --from=build /usr/local/my-cms/target/release/my-cms-api .

EXPOSE 5000
CMD ["/app/my-cms-api"]
