# My-CMS

Headless CMS: Rust/Axum/SeaORM API, React/DaisyUI/TipTap web apps, and
Supabase PostgreSQL, pgvector, and Storage. OpenAI is the product integration
for translation and embeddings. It is not the coding harness.

## Harness

Claude Code with AWS AI-DLC is the only agent harness. Agent definitions are
the installed `.claude/agents/aidlc-*` files. Do not add a second role-agent
set, and do not recreate removed Codex or OpenCode agents.

Start structured work with `/aidlc`. Method files live in
`aidlc/spaces/default/memory/`. Project contracts live in
`aidlc/spaces/default/knowledge/aidlc-shared/`. Narrative docs live in
`docs/`. There is no OpenSpec tree.

The main Claude Code session coordinates. Installed AI-DLC agents do the
stage work. One writer at a time for any shared source file or intent
artifact. A contributor who does not own a file returns guidance instead of
editing it.

## Operating rules

- Preserve unrelated working-tree changes. Do not use destructive Git
  commands. Do not hand-edit generated SeaORM entities.
- Every handoff states **Goal**, **Context**, **Constraints**, **Done when**,
  evidence, decisions, risks, and the next owner.
- Separate product, design, architecture, implementation, and release
  decisions. Do not invent scope to finish an implementation. Return a
  material conflict to the person running the workflow.
- Implementation of behavior is test-first: RED, GREEN, REFACTOR. Mark work
  done only after the stated check passes.
- If a code-review graph is unavailable, say so and use targeted search,
  `git diff`, and tests. Do not invent graph findings.
- Production or other environment-changing deploy commands run only after
  explicit user or release authorization. A ready plan is not authorization.
  Without it, report readiness, rollback trigger, and the approving owner.
  Do not describe the change as deployed.

A fast fix is allowed only when all of these hold: one non-generated file,
at most 40 non-generated lines, no observable behavior or operator change,
no auth, API, schema, migration, dependency, deployment, secret, or generated
change, and a focused verification command. Otherwise use `/aidlc`.

## Engineering conventions

### Rust API

- Gateway (`apps/api/gateway/`) composes HTTP and process concerns. Domain
  crates (`apps/api/domain_*/`) own handlers, services, adapters, entities,
  and migrations. Business logic stays in domain handlers.
- Return `Result<T, AppError>` and propagate with `?`. No production
  `unwrap()` or `expect()`.
- Schema-first SeaORM: migration, run it, generate entities. Use transactions
  for atomic writes.
- `#[instrument]` on important paths. `info!`, `warn!`, and `error!` at the
  right level. Do not block inside Tokio async work.
- Prefer SeaORM `MockDatabase` unit tests and `testcontainers` PostgreSQL
  integration tests.

### React web

- Admin pages in `apps/web/src/app/admin/` own fetching and routing. Reusable
  presentational components live in `src/components/`.
- React Hook Form with Zod, `useAuth()` for auth, Apollo for GraphQL, and
  `authenticatedFetch(getApiUrl(...))` for REST.
- DaisyUI, Tailwind, Lucide, and Sonner as already used. Cover loading, empty,
  validation, error, destructive, keyboard, focus, and responsive states.
- Public reader is `apps/ducth-dev-website`. Shared article rendering is
  `packages/editor-prose`.

## Verification

Run the targeted tests for the change. Before claiming an implementation
complete, run the repository gate and report any pre-existing failure
separately:

```bash
cargo check
cargo test
cargo fmt -- --check
cargo clippy
pnpm --dir apps/web build
```

Also build and test `apps/ducth-dev-website` and `packages/editor-prose` when
the reader or prose contract changes. `aidlc doctor` checks the harness, not
the API.
