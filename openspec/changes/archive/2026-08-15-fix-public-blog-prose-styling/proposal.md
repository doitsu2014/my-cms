## Why

A user reported that posts created in the admin site render on the public reader
(`apps/ducth-dev-website`) but the article body looks unstyled: typography,
links, blockquotes, lists, code blocks, tables, and images all fall back to the
browser default instead of the ink-tide design language. The current
`.article-prose` rule set in `apps/ducth-dev-website/src/App.css` only covers
the basic TipTap output (headings, paragraphs, links, blockquotes, simple
lists, inline code, preformatted code) and is missing the styles for tables,
task lists, underlined/strikethrough/highlighted/superscript/subscript inline
marks, YouTube embeds, standalone images, and the code-block class that
`@tiptap/extension-code-block-lowlight` emits. The spec at
`openspec/specs/website-design-system/spec.md` already requires the
`.article-prose` contract to style "headings, paragraphs, links, blockquotes,
lists, inline code, code blocks, separators, pull quotes, and images" — the
implementation simply does not deliver on the "images", the table extensions,
or the inline marks the editor can produce.

## What Changes

- Extend the `.article-prose` rule set in `apps/ducth-dev-website/src/App.css`
  so the contract covers every TipTap node the admin editor can emit (tables
  with `tiptap-table` class, task list items with `data-type="taskList"`,
  `<u>`, `<s>`, `<mark>`, `<sub>`, `<sup>`, the `text-primary underline`
  links, the `rounded max-w-full h-auto` images, the YouTube iframe wrapper,
  and the `code-block` pre class).
- Keep the spec constraint that the prose contract **SHALL NOT depend on
  Tailwind Typography** — no `prose`/`prose-lg` classes on `ArticleProse`,
  no `--tw-prose-*` variables, no `@plugin '@tailwindcss/typography'`
  reliance for the article body.
- Sync the design notes under `apps/ducth-dev-website/design/02-typography.md`
  and `08-components.md` so the prose row matches the canonical spec (custom
  `.article-prose` contract) instead of the outdated "uses `prose prose-lg`"
  wording. This is a docs-only fix.
- Strengthen the post-page test in
  `apps/ducth-dev-website/src/pages/pages.test.tsx` so a TipTap document
  with a table row, a task list item, an inline `<u>`/`<mark>`, and an image
  is asserted to be wrapped in `.article-prose`.

No breaking changes. No new dependencies. No JSX changes to `ArticleProse` —
it stays a single `<div className="article-prose">` so the spec wording
"article body SHALL use the `.article-prose` contract" continues to hold.

## Capabilities

### New Capabilities

_None._ The contract already exists; we are completing its coverage.

### Modified Capabilities

- `website-design-system`: tighten the "Focus and prose styles" Requirement so
  its Scenarios enumerate the TipTap elements the contract must style
  (headings, paragraphs, links, blockquotes, lists incl. task lists, inline
  marks, inline code, code blocks, tables, separators, pull quotes, images,
  embeds). The "SHALL NOT depend on Tailwind Typography" line stays.
- `website-reader-experience`: add a Scenario under "Post reading experience"
  that asserts the article body contains a TipTap document and the body
  carries the `.article-prose` class so the new CSS can be regression-tested.

## Impact

- `apps/ducth-dev-website/src/App.css` — additive CSS rules; no removals.
- `apps/ducth-dev-website/src/pages/pages.test.tsx` — extra assertions.
- `apps/ducth-dev-website/design/02-typography.md`,
  `apps/ducth-dev-website/design/08-components.md` — single-line edits to
  reconcile the docs with the canonical spec.
- No API / database / TipTap-editor / build-config changes. No new npm
  dependencies. No changes to the admin app.
- A prior attempt to add Tailwind Typography (`prose prose-lg` plus
  `--tw-prose-*` variables) was rejected because it violated
  `website-design-system` and was fully reverted before this change was
  scaffolded.