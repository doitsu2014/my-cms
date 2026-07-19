FROM rust:1.97 as build

WORKDIR /usr/local/my-cms

COPY . .

RUN cargo build --release --workspace


FROM rust:1.97-slim
WORKDIR /app
COPY --from=build /usr/local/my-cms/target/release/my-cms-api .
COPY --from=build /usr/local/my-cms/target/release/migration .

EXPOSE 8989
CMD ["/app/my-cms-api"]
