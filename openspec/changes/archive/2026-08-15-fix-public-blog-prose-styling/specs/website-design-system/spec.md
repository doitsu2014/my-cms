## MODIFIED Requirements

### Requirement: Focus and prose styles

The system SHALL define a single visible focus ring (2px outline, 3px offset)
used by every interactive element. The system SHALL define a `.article-prose`
contract that styles every element that the TipTap editor in `apps/web` can
emit inside the article body — at minimum: headings, paragraphs, links,
blockquotes (including the optional `data-type` cite), ordered and unordered
lists, task lists, inline marks (`<u>`, `<strong>`, `<em>`, `<s>`, `<mark>`,
`<sub>`, `<sup>`, color and highlight spans), inline code, code blocks
(including the `code-block` class from `@tiptap/extension-code-block-lowlight`),
separators (`<hr>`), pull quotes (`.pull`), tables (`<table>`, `<thead>`,
`<tbody>`, `<tr>`, `<th>`, `<td>`), images, and YouTube embeds. The prose
contract SHALL NOT depend on Tailwind Typography.

#### Scenario: Focus ring is visible and consistent

- **WHEN** any interactive element receives keyboard focus
- **THEN** it shows a 2px outline with 3px offset using the accent color

#### Scenario: Article prose renders TipTap output

- **WHEN** a post page renders a TipTap document
- **THEN** headings, paragraphs, lists, code blocks, and blockquotes use the
  prose contract styles and the existing Highlight.js code highlighting is
  preserved

#### Scenario: Tables inherit the prose contract

- **WHEN** the article body contains a TipTap table (`<table class="tiptap-table">`
  with `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>`)
- **THEN** the table uses the prose contract border, padding, and surface
  colors so it visually fits the article body and the page background

#### Scenario: Task lists inherit the prose contract

- **WHEN** the article body contains a TipTap task list
  (`<ul data-type="taskList">` with `<li data-type="taskItem">`)
- **THEN** the list items keep the prose contract list spacing, the
  checkboxes remain interactive, and the checked item's text is rendered
  with the muted ink color

#### Scenario: Inline marks are styled

- **WHEN** the article body contains TipTap inline marks
  (`<u>`, `<s>`, `<mark>`, `<sub>`, `<sup>`, color spans, highlight spans)
- **THEN** each mark receives its prose contract treatment (underline,
  strikethrough, highlighted surface, subscript / superscript baseline shift,
  foreground color inherited from the inline style) without breaking the
  surrounding line height

#### Scenario: Standalone images and embeds fit the prose contract

- **WHEN** the article body contains a TipTap image
  (`<img class="rounded max-w-full h-auto">`) or a YouTube iframe
  (`<iframe class="rounded max-w-full">`) outside of a `<figure>`
- **THEN** the element still respects the prose contract: it never exceeds
  the article body's 68ch measure, keeps its rounded corners, and centers
  itself with the same vertical rhythm as figures

#### Scenario: Code block class is recognised

- **WHEN** the article body contains a TipTap code block
  (`<pre class="code-block">` followed by `<code class="hljs language-…">`)
- **THEN** the `pre` and `pre code` prose rules apply on top of the
  `code-block` class and Highlight.js so the ink-tide background, border,
  and monospace font are preserved

#### Scenario: Prose contract is independent of Tailwind Typography

- **WHEN** the article body is rendered
- **THEN** the rendered `<div class="article-prose">` does not carry the
  Tailwind Typography `prose` / `prose-lg` / `prose-sm` utilities and the
  production CSS does not bind the article body to `--tw-prose-*` variables