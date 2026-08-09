# Changelog

All notable changes to My-CMS are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- `merge-graphql-into-posts-domain`: GraphQL endpoint moved from `/graphql/{immutable,mutable}` to `/posts/graphql/{immutable,mutable}`. The post domain (`domain_posts`) is now the sole owner of the GraphQL HTTP surface — the playground handlers and `Arc<Schema>` wiring live under `apps/api/domain_posts/src/api/post/graphql/`. The mutable mount now accepts the Supabase app roles `my-headless-cms-writer` and `my-headless-cms-administrator` (the gateway's pre-change administrator-only gate was widened to writer + administrator so all three deployment modes expose identical authorization behaviour). Operators upgrading during a deploy window should set `PUBLIC_GRAPHQL_API_URL=http://localhost:8989/posts/graphql` (or rely on the new default) and remove any reverse-proxy rewrite from the old path.