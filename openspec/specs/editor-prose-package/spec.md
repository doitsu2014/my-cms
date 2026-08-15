# editor-prose-package Specification

## Purpose
TBD - created by archiving change share-editor-prose-design-package. Update Purpose after archive.
## Requirements
### Requirement: Package exposes the canonical `.article-prose` CSS contract

The system SHALL expose a workspace-local package at
`packages/editor-prose/` whose `package.json` declares the export map
below so downstream websites can import the CSS and the React
component independently:

```
"exports": {
  ".":          "./src/index.ts",
  "./styles.css": "./src/article-prose.css"
}
```

The package SHALL contain a single CSS file at
`packages/editor-prose/src/article-prose.css` that defines the
canonical `.article-prose` rule set (headings, paragraphs, links,
blockquotes, lists, inline code, code blocks, separators, pull quotes,
images, tables, task lists, and inline marks — u, s, mark, sub, sup).
The package SHALL NOT depend on Tailwind Typography and SHALL NOT
introduce a runtime JS dependency beyond React itself.

#### Scenario: Package files exist and exports resolve

- **WHEN** a downstream website adds `"editor-prose":
  "file:../../packages/editor-prose"` to its `package.json`
- **THEN** `import 'editor-prose/styles.css'` resolves to
  `packages/editor-prose/src/article-prose.css`
- **AND** `import { ArticleProse } from 'editor-prose'` resolves to the
  package's `src/index.ts` and re-exports `ArticleProse` from
  `src/ArticleProse.tsx`

#### Scenario: ArticleProse component renders the supplied HTML

- **WHEN** a consumer mounts `<ArticleProse html="<h2>Hi</h2><p>Body</p>" />`
- **THEN** the rendered output contains a `<div class="article-prose">`
  element whose inner HTML equals the supplied string
- **AND** no other wrapper element is added

### Requirement: Downstream websites SHALL consume the shared package

The system SHALL require every website that renders TipTap article
content to consume the `editor-prose` package via the `file:` protocol
in its `package.json`. The website SHALL NOT redefine the `.article-prose`
CSS rules locally; the only source of truth is the package.

#### Scenario: Reader app imports the package and drops local rules

- **WHEN** `apps/ducth-dev-website` is built
- **THEN** `apps/ducth-dev-website/src/App.css` does not declare any
  `.article-prose *` rule (the rules now live in the package)
- **AND** `apps/ducth-dev-website/src/App.tsx` (or equivalent entry
  point) imports the package CSS once at the top of the bundle

#### Scenario: Admin app preview imports the package

- **WHEN** the admin opens the "Article Preview" modal in the TipTap
  editor
- **THEN** the preview body renders through `<ArticleProse />` imported
  from the `editor-prose` package, not through a local
  `.tiptap-preview` div with `dangerouslySetInnerHTML`

### Requirement: Cross-app smoke test pins the contract

The package SHALL ship a Vitest smoke test that mounts
`<ArticleProse>` with a fixture document containing a heading, a
paragraph, a list, a blockquote, a code block, a task list item, an
inline `<u>`/`<s>`/`<mark>`, and a `<table class="tiptap-table">`. The
test SHALL assert each element is present in the rendered DOM and that
the root element carries the `article-prose` class.

#### Scenario: Fixture renders all expected elements

- **WHEN** the test fixture is mounted
- **THEN** the rendered output contains one `<h2>`, one `<p>`, one
  `<ul>`, one `<blockquote>`, one `<pre>`, one
  `<li data-type="taskItem">`, one `<u>`, one `<s>`, one `<mark>`, and
  one `<table class="tiptap-table">` inside the `.article-prose` wrapper

### Requirement: Package README documents the contract and update flow

The package SHALL ship a `README.md` that documents: the CSS contract's
scope (which TipTap nodes are styled), the consumer update flow (edit
CSS in package, both apps pull the change on next install), and the
explicit non-goals (no Tailwind Typography, no JS-only CSS injection).

#### Scenario: README exists and links to the contract

- **WHEN** the package directory is inspected
- **THEN** `packages/editor-prose/README.md` exists and contains a
  "What is in scope" section listing the styled TipTap nodes and an
  "Update flow" section that describes editing the package CSS and
  reinstalling both downstream apps

