## MODIFIED Requirements

### Requirement: Post article body uses the shared `ArticleProse` component

The post page SHALL render localized TipTap HTML through `<ArticleProse>` imported from `editor-prose`. The page SHALL NOT mount its own inline-styled `.article-prose` div with `dangerouslySetInnerHTML`; wrapping, diagram enhancement, and prose styling SHALL come from the package. Ordinary code SHALL retain Highlight.js highlighting, while Mermaid source and generated diagram presentation SHALL be excluded from that highlighter.

#### Scenario: Post page renders through the shared component
- **WHEN** a post page renders at `/:lang/posts/:slug`
- **THEN** its article body is the `<div class="article-prose">` mounted by the shared ArticleProse component
- **AND** no additional presentation root is introduced around localized article HTML between the featured image and share row

#### Scenario: Highlight.js hooks still fire
- **WHEN** a post contains ordinary and Mermaid code blocks
- **THEN** the existing page highlighting effect processes ordinary `<pre><code>` blocks inside the article body
- **AND** it skips Mermaid source before, during, and after diagram enhancement

## ADDED Requirements

### Requirement: Mermaid rendering follows localized article content

The reader SHALL render diagrams from the selected localized article content, including a translation composed solely of Mermaid-marked code without a paragraph. Existing fallback to base content SHALL remain when translated content is absent or empty. Changing article or language SHALL replace diagrams and errors with those belonging to the current content.

#### Scenario: Diagram-only translation
- **WHEN** a Vietnamese translation contains only a Mermaid block and the base article contains a different diagram
- **THEN** the Vietnamese route displays the translated source and diagram
- **AND** the base diagram does not replace it solely because the translation lacks a paragraph

#### Scenario: Route, locale, and fallback changes
- **WHEN** the reader switches article or language while diagram rendering is pending
- **THEN** only the newly selected content's diagrams or fallback are visible
- **AND** absent or empty translated content continues to use existing base-content fallback
