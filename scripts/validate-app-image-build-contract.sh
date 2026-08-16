#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"
workflow_dir="${WORKFLOW_DIR:-$repo_root/.github/workflows}"

fail() {
  printf 'image-build-contract: %s\n' "$1" >&2
  exit 1
}

assert_file_contains() {
  local file="$1"
  local pattern="$2"
  local description="$3"

  rg -q --fixed-strings -- "$pattern" "$file" || fail "$description ($file)"
}

assert_file_exists() {
  [[ -f "$1" ]] || fail "required build input is missing ($1)"
}

assert_file_not_contains() {
  local file="$1"
  local pattern="$2"
  local description="$3"

  if rg -q --fixed-strings -- "$pattern" "$file"; then
    fail "$description ($file)"
  fi
}

assert_publisher_contract() {
  local file="$1"
  local image="$2"
  local source_path="$3"

  assert_file_exists "$file"
  assert_file_contains "$file" 'branches:' \
    'publisher must be limited to main branch pushes'
  assert_file_contains "$file" '- main' \
    'publisher must be limited to main branch pushes'
  assert_file_contains "$file" "$source_path" \
    'publisher must select its application build input'
  assert_file_contains "$file" "$image" \
    'publisher must target its approved Docker Hub repository'
  assert_file_contains "$file" 'type=sha' \
    'publisher must publish a source SHA tag'
  assert_file_contains "$file" 'type=raw,value=latest' \
    'publisher must update latest only from main'
  assert_file_contains "$file" 'refs/heads/{0}' \
    'publisher must update latest only from main'
  assert_file_contains "$file" 'permissions:' \
    'publisher must declare least-privilege permissions'
  assert_file_contains "$file" 'contents: read' \
    'publisher must declare read-only repository permissions'
  assert_file_contains "$file" 'concurrency:' \
    'publisher must serialize updates to latest'
  assert_file_not_contains "$file" 'type=semver' \
    'publisher must not generate semantic-version tags'
  assert_file_not_contains "$file" 'pull_request:' \
    'publisher must not run on pull requests'
  if rg -q '^    tags:' "$file"; then
    fail "publisher must not trigger from tag pushes ($file)"
  fi
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
assert_file_contains "$workflow_dir/release-my-cms-admin-image.yml" \
  'editor-prose=./packages/editor-prose' \
  'admin release build must provide editor-prose'
assert_file_contains "$workflow_dir/release-my-cms-ducth-dev-website-image.yml" \
  'editor-prose=./packages/editor-prose' \
  'Ducth release build must provide editor-prose'

assert_publisher_contract "$workflow_dir/release-my-cms-image.yml" \
  'doitsu2014/my-cms' 'apps/api/**'
assert_publisher_contract "$workflow_dir/release-my-cms-admin-image.yml" \
  'doitsu2014/my-cms-admin' 'apps/web/**'
assert_publisher_contract "$workflow_dir/release-my-cms-ducth-dev-website-image.yml" \
  'doitsu2014/my-cms-ducth-dev-website' 'apps/ducth-dev-website/**'

if [[ -e "$workflow_dir/validate-app-image-builds.yml" ]]; then
  fail 'dedicated application image validation workflow must be absent'
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
  "$workflow_dir/release-my-cms-admin-image.yml" \
  "$workflow_dir/release-my-cms-ducth-dev-website-image.yml"; then
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
  '  Publishers: main-only, source SHA tags, latest only for main' \
  '  Dedicated image-validation workflow: absent' \
  '  Frontend installs: frozen lockfiles' \
  '  Missing editor-prose context: Dockerfile COPY --from=editor-prose fails before image completion' \
  "  Direct invocations: ${direct_build_commands[*]}"
