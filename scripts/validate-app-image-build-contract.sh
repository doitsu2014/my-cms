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

assert_release_publisher_contract() {
  local file="$1"
  local release_workflows
  local validation_line
  local login_line

  assert_file_exists "$file"
  release_workflows="$(find "$workflow_dir" -maxdepth 1 -type f -name '*.yml' -exec rg -l '^  release:' {} + | sort)"
  [[ "$release_workflows" == "$file" ]] ||
    fail 'exactly one GitHub Release image publisher workflow is required'
  assert_file_contains "$file" 'release:' \
    'release publisher must trigger from published GitHub Releases'
  assert_file_contains "$file" 'types: [published]' \
    'release publisher must trigger only after a release is published'
  assert_file_contains "$file" '^v[0-9]+\.[0-9]+\.[0-9]+$' \
    'release publisher must validate exact vX.Y.Z tags before publication'
  assert_file_contains "$file" 'fetch-depth: 0' \
    'release publisher must check out the complete release ref history'
  assert_file_contains "$file" 'contents: write' \
    'release publisher must declare only the GitHub Release-note write permission it needs'
  assert_file_contains "$file" 'cancel-in-progress: false' \
    'release publisher must serialize same-version publication without cancellation'
  assert_file_contains "$file" 'hub.docker.com/v2/repositories' \
    'release publisher must preflight existing immutable Docker Hub version tags'
  assert_file_contains "$file" 'flavor: latest=false' \
    'release publisher must explicitly suppress metadata latest tags'
  assert_file_contains "$file" 'type=raw,value=${{ needs.validate.outputs.tag }}' \
    'release publisher must publish the exact release version tag'
  assert_file_contains "$file" 'type=raw,value=sha-' \
    'release publisher must publish a source SHA tag'
  assert_file_contains "$file" 'doitsu2014/my-cms-admin' \
    'release publisher must target the approved admin Docker Hub repository'
  assert_file_contains "$file" 'doitsu2014/my-cms-ducth-dev-website' \
    'release publisher must target the approved Ducth Docker Hub repository'
  assert_file_contains "$file" 'doitsu2014/my-cms' \
    'release publisher must target the approved API Docker Hub repository'
  assert_file_contains "$file" 'context: "./apps/api"' \
    'release publisher API build must use the app-scoped context'
  assert_file_contains "$file" 'context: "./apps/web"' \
    'release publisher admin build must use the app-scoped context'
  assert_file_contains "$file" 'context: "./apps/ducth-dev-website"' \
    'release publisher Ducth build must use the app-scoped context'
  assert_file_contains "$file" 'editor-prose=./packages/editor-prose' \
    'release publisher frontend builds must provide the editor-prose named context'
  assert_file_contains "$file" '<!-- image-manifest:start -->' \
    'release publisher must upsert a marker-delimited image manifest'
  assert_file_contains "$file" 'gh release edit' \
    'release publisher must update the associated GitHub Release notes'
  assert_file_contains "$file" "status='COMPLETE'" \
    'release publisher must mark a complete image manifest explicitly'
  assert_file_contains "$file" "status='INCOMPLETE'" \
    'release publisher must mark a partial image manifest explicitly'
  validation_line="$(rg -n --fixed-strings '^v[0-9]+\.[0-9]+\.[0-9]+$' "$file" | head -n 1 | cut -d: -f1)"
  login_line="$(rg -n --fixed-strings 'uses: docker/login-action@v3' "$file" | head -n 1 | cut -d: -f1)"
  [[ -n "$validation_line" && -n "$login_line" && "$validation_line" -lt "$login_line" ]] ||
    fail "release tag validation must appear before Docker Hub login ($file)"
  if rg -q '^  push:' "$file"; then
    fail "release publisher must not trigger on pushes ($file)"
  fi
  assert_file_not_contains "$file" 'pull_request:' \
    'release publisher must not trigger on pull requests'
  assert_file_not_contains "$file" 'workflow_dispatch:' \
    'release publisher must not allow an unbound manual release publication'
  assert_file_not_contains "$file" 'type=semver' \
    'release publisher must not enable metadata semver latest behavior'
  assert_file_not_contains "$file" 'type=raw,value=latest' \
    'release publisher must never update latest'
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
assert_release_publisher_contract "$workflow_dir/publish-github-release-images.yml"

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
  "$workflow_dir/release-my-cms-ducth-dev-website-image.yml" \
  "$workflow_dir/publish-github-release-images.yml"; then
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
  '  Main publishers: main-only, source SHA tags, latest only for main' \
  '  Release publisher: published vX.Y.Z releases only, version/SHA tags, never latest' \
  '  Dedicated image-validation workflow: absent' \
  '  Frontend installs: frozen lockfiles' \
  '  Missing editor-prose context: Dockerfile COPY --from=editor-prose fails before image completion' \
  "  Direct invocations: ${direct_build_commands[*]}"
