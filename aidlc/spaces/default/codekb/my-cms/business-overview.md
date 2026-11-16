# Business Overview

## Domain and Purpose

This repository implements a content-management platform with an administrator authoring interface and a localized public reader. The backend gateway composes domain services for posts, authentication, media, users, and SEO. Project documentation identifies Supabase as the database, identity, and object-storage platform, and OpenAI as a translation and embedding provider. Evidence: `README.md`, the scanned package manifests and entry points, and the project knowledge map cited in the developer scan.

## Key Capabilities

- Administrators author and manage posts and related categories, tags, and media through `apps/web/`.
- The API exposes REST routes and a posts GraphQL surface; the public immutable and authenticated mutable GraphQL paths are documented in the scan.
- The public website serves localized home, category, post, and about pages.
- Post translation and translation-job lifecycle are implemented in the posts domain; exact behavior and external payload contracts were not exhaustively scanned.
- SEO head assets can be managed by administrators and delivered publicly when enabled.

## Product Boundary

The shipped API binary is `my-cms-api`, composed by `gateway`; `domain_auth` is not a separate deployment surface. The admin and public site are separate frontend applications, with `packages/editor-prose/` shared between them. Deployment configurations include Docker Swarm and Kubernetes/Helm.

## Evidence and Limits

This is a static, breadth-limited synthesis, not an exhaustive product specification. The scan deeply examined selected package manifests and entry points, skimmed remaining modules, and ran no tests, builds, or programs. See `reverse-engineering-timestamp.md` for exact deep and shallow coverage.