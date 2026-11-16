# Reverse Engineering Timestamp

## Run Record

- Date: 2026-11-15
- Commit: 039a5815c074a351c4ae56517932dc9793c187a0

## Scope of Analysis

```yaml
scope_version: 1
kind: partial
intent: rewrite-brds-glossary
fingerprint: a34fe9d7dcb4ffd93f0c9d81c2ab221c950d2374
analyzed:
  paths:
    - apps/api/
    - apps/web/
    - apps/ducth-dev-website/
    - packages/editor-prose/
    - deployments/docker-swarm/
    - deployments/k8s/
    - .github/workflows/
    - README.md
    - .gitignore
    - apps/web/.gitignore
    - apps/ducth-dev-website/.gitignore
  components:
    - `gateway`
    - `domain_interface`
    - `domain_posts`
    - `domain_auth`
    - `domain_media`
    - `domain_user`
    - `domain_seo`
    - `test_helpers`
    - `admin_side`
    - `ducth-dev-website`
    - `editor-prose`
    - `docker-swarm`
    - `k8s`
    - GitHub Actions workflows
shallow:
  paths:
    - remaining source modules and tests within `apps/api/`
    - remaining source modules and tests within `apps/web/`
    - remaining source modules and tests within `apps/ducth-dev-website/`
    - remaining source modules and tests within `packages/editor-prose/`
    - remaining deployment definitions in `deployments/`
    - documentation outside root `README.md`
    - remaining active workflow files in `.github/`
```

The listed analyzed paths are exactly the selected deep-scan paths, not a claim of full repository coverage. Skimmed areas are recorded as shallow. Ignored, generated, dependency, cache, and AI-DLC workspace paths were left out and are not represented as shallow coverage.