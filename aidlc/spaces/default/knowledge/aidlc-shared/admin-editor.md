# Admin editor and shared prose

Narrative: `docs/features/editor-prose-contract.md`.

Validate: `packages/editor-prose/`, `apps/web` TipTap editor and article
preview, `apps/ducth-dev-website` article body.

## Shared contract

`packages/editor-prose` owns `.article-prose` and the stylesheet both the
admin preview and the public reader use. Consumers MUST NOT fork those
rules. Admin chrome and the editable canvas stay in `apps/web`.

WHEN saved TipTap HTML gains a node, THEN the shared contract and its
fixture coverage include that markup. Downstream apps depend on the package
rather than copying CSS. The package README states the contract and the
update flow.

Highlight.js stays on ordinary code blocks. Mermaid is not inferred from
unmarked code.

## Quick Toolbar

On editable rich text, a mouse action at a resolvable position opens a
Quick Toolbar with only Add Image, Add Video, and Add Code Block.

- The toolbar keeps the insertion position captured at open. Actions insert
  there. Dismiss or cancel does not change the document.
- After it opens, it is keyboard operable and dismissible.
- Video and code-block actions reuse the existing authoring contracts.
- Read-only mode does not offer insertion.

## Images

Both image entry points (toolbar and the existing control) offer the same
existing-media picker and local upload.

- Only image assets can be selected.
- A deliberate ordered selection inserts in that order.
- Multiple local uploads continue after a failure, report partial success,
  and do not duplicate a failed file.
- The picker has loading, empty, and error states and does not block other
  authoring.
- URL insertion on the persistent toolbar remains available.

## Mermaid

Authors can insert a Mermaid block or mark the current code block as
Mermaid without dropping its text. Undo and redo restore both states.
Invalid syntax does not block save or publish. Preview does not dirty the
editor or change saved HTML.

Canonical HTML is escaped text in
`<pre><code class="language-mermaid">…</code></pre>`. Markdown interchange
uses a `mermaid` fence that cannot be closed early by backticks inside the
source. Rendered SVG, images, loading, and errors are not stored.

Article Preview and the public reader use one renderer for explicitly
marked blocks. Each block keeps readable source while loading and on
failure. One failure does not break neighbors. A render that finishes after
the content changed or the view unmounted MUST NOT overwrite newer content.

Source is untrusted. Rendering uses bundled code only, does not call an
external render service, and does not run author scripts, HTML, callbacks,
or author links. Directives cannot loosen that policy. Source over 50,000
characters or graphs over 500 edges fall back to source plus a short
message.

A rendered diagram has an accessible name and a keyboard-operable source
disclosure. Status is not color-only. At 360px, overflow stays inside the
diagram. Light, dark, and reduced motion are respected.
