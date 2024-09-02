FROM rust:1.80 as build

WORKDIR /usr/local/my-cms

COPY . .

RUN cargo build --release


FROM rust:1.80-slim
WORKDIR /app
COPY --from=build /usr/local/my-cms/target/release/my-cms-api .

EXPOSE 5000
CMD ["/app/my-cms-api"]
