## Why

The admin editor (`apps/web`) and the public reader (`apps/ducth-dev-website`)
render the **same TipTap HTML** through two completely independent style
systems. The admin's TipTap styles target `.tiptap` and `.tiptap-preview` and
are scoped to the admin app; the reader's `.article-prose` rules are scoped
to the reader app. When an author previews an article inside the admin, they
see the DaisyUI/Roboto editor surface — not the ink-tide/Noto Serif Display
that readers will actually see — so the preview lies about the published
result.

Today the canonical `.article-prose` contract lives only in
`apps/ducth-dev-website/src/App.css`. The admin cannot consume it without
copy-pasting rules, and any future drift between the two rule sets is
invisible until a reader complains.

## What Changes

- Extract the `.article-prose` CSS contract, the `<ArticleProse>` React
  component, and the responsive aspect of the reader's design tokens into
  a single new shared package at `packages/editor-prose/`.
- `apps/ducth-dev-website` imports the package, removes the duplicated
  `.article-prose` rules from its own `App.css`, and continues to render
  articles unchanged visually.
- `apps/web` (admin) imports the same package, swaps the editor preview
  modal (`tiptap-editor.tsx` "Article Preview") to use the shared
  `<ArticleProse>` component, and the admin's preview now matches the
  reader's output exactly.
- Both apps keep their own theme tokens (`--accent` for ink-tide,
  `--color-primary` for DaisyUI emerald) and their own typography
  variables; only the **structural** rule set is shared.

No breaking changes to the published reader, the admin feature surface, or
the API. No new dependencies. The package reuses existing tools (rsbuild +
pnpm + Tailwind 4) that both apps already run.

## Capabilities

### New Capabilities

- `editor-prose-package`: a single shared package at `packages/editor-prose/`
  that exports the canonical `.article-prose` CSS contract, the
  `<ArticleProse>` React component, and a documented contract for
  downstream websites to consume so their rendered TipTap content matches
  the admin preview.

### Modified Capabilities

- `website-design-system`: add a Requirement that the canonical
  `.article-prose` rule set SHALL live in the shared `editor-prose`
  package, SHALL be the single source of truth for TipTap article
  rendering, and SHALL be imported by both `apps/ducth-dev-website` and
  `apps/web` (admin preview). Existing Requirements about the contract's
  shape, the no-Tailwind-Typography rule, and the ink-tide token mapping
  are unchanged.
- `website-reader-experience`: add a Scenario under the post-page
  rendering expectations that confirms the article body is rendered by
  the shared `<ArticleProse>` component (no more direct
  `dangerouslySetInnerHTML` into an inline-styled `.article-prose` div
  inside the app).

## Impact

- `packages/editor-prose/` (new): `package.json`, `tsconfig.json`,
  `src/index.ts`, `src/ArticleProse.tsx`, `src/article-prose.css`,
  `README.md`, plus unit tests.
- `apps/ducth-dev-website/`:
  - `package.json` adds `"editor-prose": "file:../../packages/editor-prose"`.
  - `src/App.tsx` imports the package CSS once (replacing the local
    `.article-prose` block).
  - `src/components/posts/ArticleProse.tsx` becomes a re-export of the
    shared `<ArticleProse>` so existing imports keep resolving.
  - `src/App.css` deletes the `.article-prose` rule block.
  - Tests in `src/pages/pages.test.tsx` and `src/test/redesign.test.tsx`
    are updated to import the component from the package path.
- `apps/web/`:
  - `package.json` adds `"editor-prose": "file:../../packages/editor-prose"`.
  - `src/app/admin/components/inputs/rich-text-editor/tiptap-editor.tsx`
    swaps the `tiptap-preview` div to render through the shared
    `<ArticleProse>` component.
  - `src/app/admin/components/inputs/rich-text-editor/tiptap-editor.css`
    deletes the now-redundant `.tiptap-preview *` rule block.
- `openspec/specs/website-design-system/spec.md` — delta applied at
  archive time.
- `openspec/specs/website-reader-experience/spec.md` — delta applied at
  archive time.
- No DB migration. No API change. No Docker or deployment config change.
  The Dockerfile copies both apps as before.
- The shared package must be referenced via `file:` (each app already has
  its own pnpm workspace); we do not introduce a top-level `pnpm-workspace.yaml`
  to avoid scope creep.

## Open Question

The admin editor surface itself (`.tiptap` — the contenteditable where the
author actually types) keeps its DaisyUI/Roboto styles in this change.
Aligning the *editing* surface itself with the reader would mean swapping
the theme and font for the whole admin app, which is a much larger blast
radius and is deferred.
