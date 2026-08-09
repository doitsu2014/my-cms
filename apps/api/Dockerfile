FROM rust:1.97 as build

WORKDIR /usr/local/my-cms

COPY . .

# Build the workspace; only the `my-cms-api` bin is shipped to the
# runtime image. The gateway binary depends on every `domain_*` crate
# so the workspace build is required; the `--bin my-cms-api` filter
# prevents cargo from building any other `[[bin]]` target that may exist
# in workspace members (e.g. the `domain_auth` placeholder bin).
RUN cargo build --release --workspace --bin my-cms-api


FROM rust:1.97-slim
WORKDIR /app
COPY --from=build /usr/local/my-cms/target/release/my-cms-api .

EXPOSE 8989
CMD ["/app/my-cms-api"]
