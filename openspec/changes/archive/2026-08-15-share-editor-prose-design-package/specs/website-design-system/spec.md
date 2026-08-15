## MODIFIED Requirements

### Requirement: Focus and prose styles

The system SHALL define a single visible focus ring (2px outline, 3px offset) used by every interactive element. The system SHALL define a `.article-prose` contract that styles headings, paragraphs, links, blockquotes, lists, inline code, code blocks, separators, pull quotes, and images within the article body. The prose contract SHALL NOT depend on Tailwind Typography.

The `.article-prose` rule set SHALL live in the shared `editor-prose` package at `packages/editor-prose/src/article-prose.css` and SHALL be the single source of truth. Every website that renders TipTap article content SHALL consume the package; no website SHALL redefine the rule set locally.

#### Scenario: Focus ring is visible and consistent

- **WHEN** any interactive element receives keyboard focus
- **THEN** it shows a 2px outline with 3px offset using the accent color

#### Scenario: Article prose renders TipTap output

- **WHEN** a post page renders a TipTap document
- **THEN** headings, paragraphs, lists, code blocks, and blockquotes use the prose contract styles and the existing Highlight.js code highlighting is preserved

#### Scenario: Contract lives in the shared package

- **WHEN** the `.article-prose` rule block is searched for in either `apps/ducth-dev-website/src/App.css` or `apps/web/src/`
- **THEN** no match is found (the rules live in `packages/editor-prose/src/article-prose.css`)

#### Scenario: Both apps import the same package

- **WHEN** either `apps/ducth-dev-website/package.json` or `apps/web/package.json` is inspected
- **THEN** `"editor-prose": "file:../../packages/editor-prose"` is listed in `dependencies`
