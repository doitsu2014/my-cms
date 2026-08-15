## Context

`apps/ducth-dev-website/src/App.css` holds the canonical `.article-prose`
contract today: 45 selectors that style TipTap output for the reader
(`.article-prose h2`, `.article-prose blockquote`, `.article-prose
table.tiptap-table`, `.article-prose li[data-type="taskItem"]`, etc.).
The companion React component lives at
`apps/ducth-dev-website/src/components/posts/ArticleProse.tsx` — a 6-line
wrapper that puts `dangerouslySetInnerHTML` inside a single `<div
className="article-prose">`.

`apps/web/src/app/admin/components/inputs/rich-text-editor/tiptap-editor.tsx`
runs the same TipTap editor but uses 94 `.tiptap*` rules in
`tiptap-editor.css` — its own selector set, its own DaisyUI tokens, its own
Roboto font. The "Article Preview" modal renders `editor.getHTML()` inside
`<div className="tiptap-preview ...">`, so authors see the admin's
DaisyUI/Roboto surface rather than the reader's ink-tide output.

Both apps are isolated rsbuild projects (each owns its own
`pnpm-workspace.yaml` with only `allowBuilds` set). There is no top-level
pnpm workspace. Tailwind 4 with DaisyUI is configured in both apps.
PostCSS runs `@tailwindcss/postcss` + `autoprefixer`. There is no existing
shared package between the two apps — this change introduces the first.

## Goals / Non-Goals

**Goals:**

- One canonical CSS contract and React component for TipTap article body
  rendering, owned by a new shared package at `packages/editor-prose/`.
- `apps/ducth-dev-website` consumes the package; its `App.css` no longer
  carries the `.article-prose` rule block.
- `apps/web` admin consumes the same package; the editor preview modal
  renders through the shared component.
- The reader's published appearance is unchanged byte-for-byte (CSS
  selector set, variable names, and rule order preserved).
- The admin's editing surface (the contenteditable) keeps its DaisyUI /
  Roboto look-and-feel — only the preview changes.
- Both apps gain a deterministic cross-app snapshot test so any future
  divergence between the package and either consumer is caught by CI.

**Non-Goals:**

- No redesign of the ink-tide theme, the DaisyUI emerald theme, or the
  reader's typography stack.
- No new top-level pnpm workspace. Each app continues to resolve the
  package via the `file:` protocol.
- No new build pipeline. rsbuild picks up the package's `.css` and
  `.tsx` exactly the same way it picks up local files.
- No Tailwind Typography adoption. The package's contract is pure CSS
  using ink-tide tokens — same constraint as today.
- No API, no DB, no auth, no media, no deployment changes.

## Decisions

### Decision 1 — `file:` protocol, not a top-level pnpm workspace

**What:** Each app adds `"editor-prose": "file:../../packages/editor-prose"`
to its `package.json`. No top-level `pnpm-workspace.yaml` is introduced.

**Why:** Each app already runs an isolated rsbuild project with its own
`pnpm-workspace.yaml`. Adding a top-level workspace changes the resolution
context for every dependency in both apps and is well outside the scope of
this change. `file:` is supported by pnpm 10 and works without a workspace
declaration, as long as the target directory has a valid `package.json`.

**Alternatives considered:**

- *Top-level `pnpm-workspace.yaml` declaring `apps/*` and `packages/*`.*
  Rejected — bigger blast radius, affects rsbuild resolution, and breaks
  the "no changes to deployment" promise.
- *Copy the CSS file into each app at build time.* Rejected — defeats the
  whole point of the change; drifts as soon as someone edits one copy.

### Decision 2 — Export both CSS and a React component

**What:** The package's `package.json` declares two subpath exports:

```
"exports": {
  ".":          { "types": "./dist/index.d.ts", "import": "./src/index.ts" },
  "./styles.css": "./src/article-prose.css"
}
```

The default export is the `<ArticleProse>` React component. The CSS file is
imported separately by each app at its top level (after Tailwind layers).

**Why:** Both consumers want the React component for rendering and the CSS
file for class-driven descendant styling. A single default export cannot
serve both — Tailwind and PostCSS need to see the CSS file as a real asset,
not as a JavaScript import.

**Alternatives considered:**

- *Inline the styles inside the component via a `<style>` tag.*
  Rejected — duplicates the rules into every rendered instance, breaks
  caching, and breaks the "no Tailwind Typography / no @apply" rule.
- *Just the CSS file, no component.* Rejected — the admin preview still
  needs the wrapper, and copy-pasting a 6-line component into two apps
  invites the same drift this change is meant to prevent.

### Decision 3 — Move the CSS rule block verbatim

**What:** The 45 `.article-prose` selectors in
`apps/ducth-dev-website/src/App.css` are moved into
`packages/editor-prose/src/article-prose.css` **without modification**.
Variable names, ordering, comments, and the rule set's location inside the
file are preserved.

**Why:** The change is explicitly about *consolidating* the contract, not
redesigning it. Any visual change would invalidate the snapshot test in
`apps/ducth-dev-website/src/pages/pages.test.tsx` and force every reader
page to re-render. Keeping the CSS identical makes the change low-risk and
makes the diff trivial to review.

**Alternatives considered:**

- *Tidy up the CSS while moving it.* Rejected — out of scope. Tidy-up
  belongs in a follow-up change.

### Decision 4 — Admin preview wraps the shared component

**What:** `tiptap-editor.tsx` keeps its modal, the toolbar, and the HTML
edit modal exactly as today. Only the body of the "Article Preview" modal
changes: the `<div className="tiptap-preview ..." dangerouslySetInnerHTML=
{{ __html: editor.getHTML() }} />` becomes `<ArticleProse html=
{editor.getHTML()} />` from the shared package.

**Why:** This is the minimum viable edit that delivers WYSIWYG honesty for
the author. The editing surface (`.tiptap`), the toolbar, the fullscreen
mode, and the HTML edit modal are untouched and continue to render with
DaisyUI tokens.

**Alternatives considered:**

- *Re-theme the whole `.tiptap` editing surface with ink-tide.* Deferred
  to a follow-up — too large for this change.
- *Use an `<iframe>` to embed the reader's actual rendered output.* 
  Rejected — over-engineered, and a same-origin iframe does not deliver
  anything the in-document component does not.

### Decision 5 — Snapshot test pins the contract

**What:** The package ships a Vitest snapshot test that captures the
serialized `.article-prose` CSS string. Both apps add a tiny smoke test
that imports the package, mounts an `<ArticleProse>` with a small fixture
HTML, and asserts the rendered DOM matches a snapshot. CI fails on any
drift.

**Why:** Today the contract is verified only by the reader's
`pages.test.tsx` checking structural class names. After the move, we want
both consumers to fail fast if the package is edited without coordination.

**Alternatives considered:**

- *Just rely on visual QA.* Rejected — visual QA cannot catch a regression
  where someone edits the CSS in only one consumer.

## Risks / Trade-offs

- **[Risk] Two apps importing the same CSS file via `file:` may load it
  twice in the browser if both apps are reachable from the same tab.**
  → Both apps are isolated routes served from different origins in
  production; this is not a real-world collision. In dev, each app loads
  its own copy. No mitigation needed.
- **[Risk] rsbuild does not pick up the package's CSS via `file:`.**
  → Mitigation: a tiny smoke test asserts the rendered class names exist
  in the built output. If rsbuild chokes, the smoke test fails before
  merge.
- **[Risk] Drift between the two apps' `package.json` versions of the
  package.** → The package has no version, only a path. Pnpm pins the
  path at install time. We document in the package `README.md` that
  bumping requires editing both apps in the same commit.
- **[Risk] Tailwind purges `article-prose` class names from generated
  utility CSS because the content scanner does not see them.** → Not a
  concern: `article-prose` is a CSS selector used in JSX, so the
  scanner finds it. The descendant rules in `article-prose.css` are not
  Tailwind utilities and are never subject to purging.
- **[Risk] Snapshot test flakes on whitespace differences.** → Mitigation:
  use `toMatchInlineSnapshot` and pin the CSS via a string equality check,
  not a snapshot. Whitespace does not affect behavior.

## Migration Plan

1. Land the new package and the reader's import switch in one PR. The
   reader's published pages stay byte-identical.
2. Land the admin preview swap in a follow-up PR so the change is easy to
   bisect.
3. Rollback: revert either PR independently. The package can be removed
   without affecting the reader or admin (they fall back to their local
   copies if we keep the deletion branches).

## Open Questions

- Should the package carry a tiny standalone Storybook so designers can
  preview the contract outside the reader? Deferred — would add a build
  dependency we don't yet have.
- Should we add a `@tailwindcss/typography`-free `.prose` variant for
  downstream sites that want Tailwind Typography semantics? Deferred —
  not yet requested by any downstream.
