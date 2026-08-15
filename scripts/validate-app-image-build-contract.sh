#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

fail() {
  printf 'image-build-contract: %s\n' "$1" >&2
  exit 1
}

assert_file_contains() {
  local file="$1"
  local pattern="$2"
  local description="$3"

  rg -q --fixed-strings "$pattern" "$file" || fail "$description ($file)"
}

assert_file_exists() {
  [[ -f "$1" ]] || fail "required build input is missing ($1)"
}

assert_file_contains apps/api/.dockerignore 'target/' \
  'API build context must exclude target/'

for dockerfile in apps/web/Dockerfile apps/ducth-dev-website/Dockerfile; do
  count="$(rg -F -c 'COPY --from=editor-prose / /packages/editor-prose' "$dockerfile")"
  [[ "$count" -ge 2 ]] || fail "$dockerfile must stage editor-prose in deps and builder"
done

assert_file_contains apps/web/Dockerfile 'COPY package.json pnpm-lock.yaml ./' \
  'admin Dockerfile must copy its committed lockfile'
assert_file_contains apps/web/Dockerfile 'pnpm install --frozen-lockfile' \
  'admin Dockerfile must use frozen-lockfile installation'
assert_file_contains apps/ducth-dev-website/Dockerfile 'pnpm install --frozen-lockfile' \
  'Ducth Dockerfile must use frozen-lockfile installation'
assert_file_contains packages/editor-prose/.dockerignore 'node_modules/' \
  'editor-prose context must exclude node_modules/'
assert_file_contains packages/editor-prose/.dockerignore '.env.*' \
  'editor-prose context must exclude environment files'

assert_file_contains deployments/docker-swarm/apps/docker-compose.yaml \
  'context: ../../../apps/web' 'Compose admin build must remain app-scoped'
assert_file_contains deployments/docker-swarm/apps/docker-compose.yaml \
  'context: ../../../apps/ducth-dev-website' 'Compose Ducth build must remain app-scoped'
assert_file_contains deployments/docker-swarm/apps/docker-compose.yaml \
  'editor-prose: ../../../packages/editor-prose' \
  'Compose frontend builds must provide editor-prose'
assert_file_contains .github/workflows/release-my-cms-admin-image.yml \
  'editor-prose=./packages/editor-prose' \
  'admin release build must provide editor-prose'
assert_file_contains .github/workflows/validate-app-image-builds.yml \
  'my-cms-api:validation' 'validation workflow must label the API image'
assert_file_contains .github/workflows/validate-app-image-builds.yml \
  'my-cms-admin:validation' 'validation workflow must label the admin image'
assert_file_contains .github/workflows/validate-app-image-builds.yml \
  'ducth-dev-website:validation' \
  'validation workflow must label the Ducth image'
if rg -q 'docker/login-action|push:[[:space:]]*true|secrets\.' \
  .github/workflows/validate-app-image-builds.yml; then
  fail 'validation workflow must not log in, push, or require secrets'
fi

for build_input in \
  apps/api/Dockerfile \
  apps/web/Dockerfile \
  apps/ducth-dev-website/Dockerfile \
  packages/editor-prose/package.json \
  packages/editor-prose/pnpm-lock.yaml; do
  assert_file_exists "$build_input"
done

if rg -q 'context:[[:space:]]+\.$' \
  deployments/docker-swarm/apps/docker-compose.yaml \
  .github/workflows/release-my-cms-admin-image.yml \
  .github/workflows/validate-app-image-builds.yml; then
  fail 'an image build must not use the repository root as its primary context'
fi

direct_build_commands=(
  'docker build -f apps/api/Dockerfile apps/api' \
  'docker build --build-context editor-prose=packages/editor-prose -f apps/web/Dockerfile apps/web' \
  'docker build --build-context editor-prose=packages/editor-prose -f apps/ducth-dev-website/Dockerfile apps/ducth-dev-website'
)
for primary_context in "${direct_build_commands[@]}"; do
  [[ "$primary_context" == *' apps/api' || "$primary_context" == *' apps/web' || "$primary_context" == *' apps/ducth-dev-website' ]] ||
    fail "unexpected primary build context in contract: $primary_context"
done

printf '%s\n' \
  'image-build-contract: PASS' \
  '  API primary context: apps/api (target/ excluded)' \
  '  Admin primary context: apps/web + editor-prose named context' \
  '  Ducth primary context: apps/ducth-dev-website + editor-prose named context' \
  '  Frontend installs: frozen lockfiles' \
  '  Missing editor-prose context: Dockerfile COPY --from=editor-prose fails before image completion' \
  "  Direct invocations: ${direct_build_commands[*]}"
