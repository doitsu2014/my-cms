# Authentication, users, and media

[Glossary](glossary.md) · Related contract: [backend domain architecture](backend-domain-architecture.md)

## Evidence boundary

This is the intended contract from existing project knowledge, supported by a partial static CodeKB scan and maintained deployment notes. Validate against `apps/api/domain_auth/`, `apps/api/domain_user/`, `apps/api/domain_media/`, relevant `apps/web` auth/user/media UI, and `deployments/docker-swarm/`. These areas were not all deeply inspected during this rewrite; claims marked **Unverified** require source confirmation.

## Authentication

- The intended identity provider is Supabase GoTrue; protected API calls use Supabase JWTs, and the admin client sends bearer credentials for authenticated operations.
- Service-role credentials are server-only. Authentication failures must not expose secrets.
- `SUPABASE_URL` and the internal service URL are described in existing knowledge as separate public and in-container endpoints. Exact configuration names, factory failure behavior, and runtime validation are **Unverified** pending source review.
- Local sign-up policy and administrator seed behavior are deployment setup, not assumptions about production policy; verify current Compose/GoTrue configuration before relying on them.

## Administrator users

Existing knowledge specifies administrator-only user operations: `POST /users`, `GET /users`, `GET /users/:id`, `PUT /users/:id`, `DELETE /users/:id`, and `POST /users/:id/reset-password`. The partial scan does not establish complete route registration, request/response shapes, validation, or authorization; treat this route list as **Unverified intended contract**.

Intended constraints include normalized user output, validated email/role/password/ban/profile values, `full_name` in user metadata, phone in GoTrue's top-level phone field with strict E.164 formatting, and saving a phone without enabling phone authentication. A reset response may return a temporary password only once; self-delete is rejected; successful mutations are auditable; upstream errors must not leak credentials. Each behavior needs source verification before being asserted as current implementation.

## Media and bucket management

The intended media adapter uses Supabase Storage with server-side service-role credentials and an explicit bucket, falling back to configured `MEDIA_BUCKET` only when no override is given. Bucket administration is administrator-only. Existing contract expectations include private-by-default creation, reserved default-bucket protection, and explicit handling for empty versus delete/purge operations. Names, cache behavior, visibility rules, response URL construction and exact API surface remain **Unverified** by the partial source scan.

## Image transformation

Existing knowledge describes resized reads as proxying transformed bytes from Supabase Image Transformation, not redirecting clients to an internal render URL. Dimension handling, upstream error mapping, headers, bucket forwarding, and cache keys are **Unverified** pending deep inspection of `domain_media` handlers and tests. Do not use the superseded server-side image decoding/redirect design.

## Open evidence items

- **Unverified:** exact auth algorithm/claims, route registration, user validation and audit behavior, media visibility/cache rules, bucket constraints and resize semantics.
- If source and this intended contract disagree, document each claim and its source path under an explicit **Unresolved** item; require project-owner decision when source alone cannot settle intended policy.
