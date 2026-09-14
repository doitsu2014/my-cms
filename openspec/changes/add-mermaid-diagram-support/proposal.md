## Why

Authors need to explain systems and workflows with diagrams inside CMS articles. The admin editor and public website currently display language-tagged code as text; adding Mermaid rendering lets authors maintain diagrams as editable source and lets readers see the resulting diagram without a separate image-authoring workflow.

## What Changes

- Add the Mermaid rendering library and support explicitly marked Mermaid code blocks in the admin editor and public article body.
- Let authorized authors insert or designate a Mermaid block, edit its source using the existing code-block workflow, and see the diagram in the editor's Article Preview. Keep source editing available after preview, save, and reopening an article.
- Render the same saved Mermaid source consistently on the public website for the selected article language, including existing articles that already contain Mermaid-marked blocks.
- Preserve Mermaid source through existing HTML editing and Markdown import/copy workflows. Markdown uses fenced `mermaid` blocks for interchange; the existing HTML article format remains authoritative.
- Show a readable source fallback and a concise failure state when a diagram is invalid or cannot render. A failing diagram must not break editing, saving, other diagrams, or the rest of the article; corrected source can render on the next preview or content update.
- Preserve ordinary code highlighting and prose styling. Diagrams remain readable within the article layout at narrow widths and expose a text alternative/source to readers using assistive technology.

### Scope and Non-goals

In scope are admin article authors and public article readers, diagram source editing, Article Preview, public rendering, existing content compatibility, and safe failure/recovery. Existing editing, publishing, and read-only permissions continue to apply. Rendering does not require an additional permission.

Out of scope are a visual diagram builder, live inline diagram previews while typing, AI diagram generation, image/PDF export, diagram asset management, a content-format migration, and new API or publishing workflows. Author-supplied executable callbacks, interactive links, arbitrary HTML, or security configuration overrides are not supported diagram features.

## Capabilities

### New Capabilities

- `mermaid-diagrams`: Author, preserve, preview, and read Mermaid diagrams with safe rendering, accessible source access, isolated failures, and recovery.

### Modified Capabilities

- `editor-prose-package`: Extend shared article presentation to render Mermaid-marked content in both consumers while retaining the `.article-prose` contract. Amend the React-only runtime restriction and exact supplied-HTML rendering requirement to permit diagram rendering, with unchanged ordinary content behavior.
- `website-reader-experience`: Render Mermaid diagrams in localized article bodies, exclude their source from ordinary code highlighting, and retain readable content before rendering and on failures.

## Impact

- Admin TipTap editor, code-block toolbar, Article Preview, and existing Markdown/HTML interoperability in `apps/web`.
- Shared article presentation and styles in `packages/editor-prose`, plus the website article highlighting integration in `apps/ducth-dev-website`.
- Frontend dependency manifests/lockfiles and focused editor, shared-renderer, and reader tests. No API contract, database schema, or stored-article migration is expected.

### Decisions, Assumptions, and Dependencies

- “Website” means the repository's public reader, `apps/ducth-dev-website`; “admin editor” includes its existing Article Preview modal. This is a proposed scope, not recorded stakeholder approval.
- The source is an explicitly Mermaid-marked code block in existing article HTML. Ordinary code is never guessed to be a diagram. Existing Markdown import/copy remains an interchange path.
- Mermaid syntax errors do not create a new save or publish gate. Authors can retain work in progress; readers get a safe fallback.
- The architect selects and verifies a compatible Mermaid dependency and the supported secure configuration. Designer guidance should fit the existing toolbar, preview, and reader layout.

### Risks

- Mermaid content must not execute scripts, invoke callbacks, navigate via author-defined links, or relax the application's rendering policy.
- Dependency loading and diagram complexity can affect responsiveness. Missing runtime assets, invalid input, and rendering failures need bounded, local fallbacks.
- Preview rendering must not replace the editable/persisted source with generated markup. Shared rendering must preserve website server rendering and hydration behavior.

### Acceptance Outcomes

1. An authorized author can insert a Mermaid flowchart or sequence diagram, edit it, preview it, save, reopen, and preview again without losing source or its Mermaid designation.
2. Existing HTML editing and fenced Markdown import/copy preserve diagram meaning, labels, and Mermaid designation across a round trip.
3. The public article displays the same diagrams as Article Preview, including multiple diagrams and the selected translated content. Existing marked articles require no resave.
4. Invalid syntax or a rendering/load failure leaves readable source and the rest of the article usable; a valid neighboring diagram still renders, and a corrected diagram can recover.
5. Ordinary code blocks remain highlighted, ordinary prose remains unchanged, and generated diagrams never enter saved article content.
6. Diagrams and fallback/source controls are usable by keyboard and assistive technology; a wide diagram does not cause whole-page horizontal overflow at a 360px viewport.
7. Server-rendered articles remain readable before browser diagram rendering, hydrate without mismatch warnings, and provide source when JavaScript is unavailable. Articles without diagrams do not load the Mermaid runtime.
