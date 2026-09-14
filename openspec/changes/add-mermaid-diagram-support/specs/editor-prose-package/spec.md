## MODIFIED Requirements

### Requirement: Package exposes the canonical `.article-prose` CSS contract

The system SHALL expose a workspace-local package at `packages/editor-prose/` whose `package.json` retains the following exports so consumers can import CSS and the React component independently:

```json
"exports": {
  ".": "./src/index.ts",
  "./styles.css": "./src/article-prose.css"
}
```

The package SHALL contain the canonical `.article-prose` rules in `packages/editor-prose/src/article-prose.css` for headings, paragraphs, links, blockquotes, lists, inline code, code blocks, separators, pull quotes, images, tables, task lists, inline marks (u, s, mark, sub, sup), and Mermaid presentation. The package SHALL NOT depend on Tailwind Typography. In addition to React it SHALL permit the Mermaid runtime and its transitive dependencies only for lazy browser diagram enhancement. Ordinary content SHALL retain the supplied HTML behavior and the single `.article-prose` root. Only explicitly marked Mermaid blocks SHALL be enhanced after browser mounting; the source HTML SHALL remain the server-rendered and initial hydration representation.

#### Scenario: Package files exist and exports resolve
- **WHEN** a consumer adds `"editor-prose": "file:../../packages/editor-prose"` to its dependencies
- **THEN** `import 'editor-prose/styles.css'` resolves to `src/article-prose.css` in the package
- **AND** `import { ArticleProse } from 'editor-prose'` resolves through `src/index.ts` to `src/ArticleProse.tsx`

#### Scenario: ArticleProse component renders the supplied HTML
- **WHEN** a consumer mounts `<ArticleProse html="<h2>Hi</h2><p>Body</p>" />`
- **THEN** the rendered root is `<div class="article-prose">` whose inner HTML equals the supplied string
- **AND** no other root wrapper is added and the Mermaid runtime is not loaded

#### Scenario: Lazy enhancement preserves server rendering
- **WHEN** ArticleProse contains Mermaid source and is rendered on the server then hydrated
- **THEN** readable source is emitted without accessing browser globals or importing the Mermaid runtime on the server
- **AND** the first client render matches the server HTML before diagram enhancement begins
- **AND** JavaScript-disabled readers retain readable source

#### Scenario: Multiple consumers share presentation
- **WHEN** the admin Article Preview and public article render the same Mermaid source
- **THEN** both use the shared renderer and stylesheet with the same security, resource, source-access, and failure behavior
