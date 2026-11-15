# Admin editor and shared prose

[Glossary](glossary.md) · Maintained narrative: [`docs/features/editor-prose-contract.md`](../../../../../docs/features/editor-prose-contract.md)

## Evidence boundary

Intended contract references `packages/editor-prose/`, the admin TipTap editor and preview in `apps/web`, and the public article renderer. The CodeKB scan is partial; details below are preserved from project knowledge and maintained documentation but are **Unverified** as current implementation unless directly source-checked.

## Shared rendered article

`packages/editor-prose` owns the `.article-prose` reader wrapper and shared stylesheet. Admin preview and public reader consume it; application-specific editor canvas and shell styles remain in `apps/web`. When an authoring extension changes persisted HTML, update shared rendering support and fixtures in the same change. Ordinary code blocks use Highlight.js; Mermaid rendering is explicit and not inferred from unmarked code.

## Authoring controls and images

The Quick Toolbar is intended to expose Add Image, Add Video, and Add Code Block at a resolvable editable position. It retains the insertion position captured when opened; dismissal/cancel does not modify content; read-only mode offers no insertion. Image entry points share an image-only existing-media picker and local upload, preserve deliberate selection order, and continue remaining uploads after an individual failure with partial-success feedback. URL insertion remains available on the persistent toolbar. Current behavior and accessibility coverage are **Unverified**.

## Mermaid content

The intended canonical saved form is escaped text in `<pre><code class="language-mermaid">…</code></pre>`; Markdown interchange uses a `mermaid` fence. Authors can insert or mark a block without losing text; undo/redo restores both states. Invalid syntax does not block save/publish; preview does not mutate saved HTML. Rendered SVG, loading state, and errors are not persisted.

Preview and reader use one renderer for explicitly marked blocks. Source stays readable during loading/failure; one bad diagram must not break neighbors or let stale async rendering overwrite changed/unmounted content. Rendering is local/bundled only and must not execute author scripts, HTML, callbacks, links, or external rendering services. The existing intended limits are 50,000 source characters and 500 graph edges, with source fallback. Accessibility includes named diagrams, keyboard source disclosure, non-color-only status, contained overflow at 360px, light/dark support and reduced-motion respect. These implementation specifics remain **Unverified** pending deep package and consumer tests.

## Open evidence items

- **Unverified:** exact toolbar behavior, upload failure handling, canonical serialization, renderer security/limits, and preview parity.
- The maintained editor guide describes the package boundary but not every newer toolbar/Mermaid requirement. Treat its statements as supporting context; resolve source/contract conflicts explicitly rather than silently preferring either prose source.
