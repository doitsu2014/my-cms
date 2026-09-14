## ADDED Requirements

### Requirement: Authors can create and preview editable Mermaid source

The admin article editor SHALL offer a keyboard-accessible Mermaid action that creates a Mermaid code block or designates the current code block as Mermaid without discarding its text. Authors SHALL edit source in the existing editor and view rendered flowcharts and sequence diagrams in Article Preview. Existing editing and read-only permissions SHALL apply. Invalid diagram syntax SHALL NOT prevent saving or publishing.

#### Scenario: Insert, edit, preview, and reopen
- **WHEN** an authorized author inserts a Mermaid block, edits a valid flowchart or sequence diagram, opens Article Preview, saves, and reopens the article
- **THEN** the preview displays the diagram and the reopened editor retains editable source and its Mermaid designation
- **AND** preview rendering does not change saved content or mark the editor dirty

#### Scenario: Designate an existing block
- **WHEN** the author invokes the Mermaid action inside an ordinary code block
- **THEN** its language becomes Mermaid and its source text remains intact
- **AND** undo and redo restore the prior and resulting block states

#### Scenario: Read-only and invalid drafts
- **WHEN** the editor is read-only
- **THEN** Mermaid insertion and source modification are unavailable
- **AND** when an editable article contains invalid Mermaid, existing save and publish actions remain available under their existing permissions

### Requirement: Diagram source round trips through existing formats

Canonical article HTML SHALL represent diagrams as escaped text in `<pre><code class="language-mermaid">…</code></pre>`. HTML editing, save/reload, Markdown import, and Copy as Markdown SHALL preserve Mermaid designation, labels, meaningful whitespace, and diagram meaning. Markdown interchange SHALL use a `mermaid` fence. Rendered images, SVG, loading state, and errors SHALL NOT be persisted or exported as diagram source.

#### Scenario: HTML and Markdown round trip
- **WHEN** Mermaid containing multiline labels, indentation, Unicode, angle brackets, ampersands, or backticks passes through HTML editing and Markdown copy/import
- **THEN** its decoded source and diagram meaning are retained without double escaping, truncation, or loss of the language marker
- **AND** exported fences cannot be prematurely terminated by backticks inside source

### Requirement: Explicit diagrams render consistently with isolated failure

Article Preview and public article presentation SHALL use the same renderer for explicitly marked Mermaid blocks. Unmarked code and other languages SHALL NOT be inferred as diagrams. Each block SHALL retain readable source while loading and on empty input, unsupported syntax, syntax error, complexity rejection, or runtime failure. Failure SHALL show a concise local status, leave adjacent prose and diagrams usable, and permit recovery on a new preview or content update.

#### Scenario: Mixed content and existing articles
- **WHEN** an article contains two valid Mermaid blocks, one invalid block, and ordinary code
- **THEN** both valid diagrams render independently, the invalid block exposes source and an error, and ordinary code retains its existing behavior
- **AND** an existing stored Mermaid-marked block works without resaving the article

#### Scenario: Failure and recovery
- **WHEN** diagram loading fails or the source is empty, unsupported, or invalid
- **THEN** source remains readable and only that block shows the failure state
- **AND** replacing it with valid source or reopening preview after runtime recovery allows another render attempt

#### Scenario: Stale rendering completion
- **WHEN** content changes or the component unmounts before rendering finishes
- **THEN** the previous result cannot overwrite newer content, mutate another article, or leave orphaned rendering elements

### Requirement: Diagram rendering enforces application security and resource policy

Diagram source SHALL be treated as untrusted text. Rendering SHALL NOT execute author scripts, callbacks, HTML, or author-defined navigation. Source-provided configuration SHALL NOT weaken the application's rendering or sanitization policy. Rendering SHALL use bundled local code and SHALL NOT send diagram source to an external rendering service. Oversized source and excessive graph edges SHALL produce a local fallback under application-controlled limits.

#### Scenario: Hostile labels and links
- **WHEN** Mermaid contains script-like labels, HTML event handlers, JavaScript URLs, or click callbacks
- **THEN** these cannot execute, invoke a callback, or create an actionable diagram link in either consumer
- **AND** source remains safely displayed as text

#### Scenario: Configuration cannot override policy
- **WHEN** diagram directives or frontmatter attempt to enable loose security, HTML labels, custom CSS, or unsafe sanitization
- **THEN** application policy remains unchanged and unsupported configuration produces safe source fallback
- **AND** a subsequent valid diagram uses the same application policy

#### Scenario: Resource limits
- **WHEN** a diagram exceeds 50,000 source characters or 500 graph edges
- **THEN** rendering refuses that diagram with readable source and a concise size or complexity message
- **AND** saving the source and reading the rest of the article remain possible

### Requirement: Diagram presentation remains accessible and responsive

Rendered diagrams SHALL have an accessible name and keyboard-operable access to their original source. Loading and error states SHALL be understandable without color alone. Diagram presentation SHALL remain readable in the supported light and dark themes, respect reduced motion, and contain overflow within the diagram at a 360px viewport.

#### Scenario: Keyboard and text alternative
- **WHEN** a keyboard or assistive-technology user reaches a diagram
- **THEN** its accessible name identifies it as a diagram and a labeled source disclosure exposes the complete source
- **AND** focus remains visible and opening or closing the disclosure does not trap keyboard focus

#### Scenario: Narrow layout and themes
- **WHEN** a wide diagram is viewed at 360px or the page changes between light and dark themes
- **THEN** the diagram remains readable and does not introduce whole-page horizontal scrolling
- **AND** overflowing source or diagram content is locally scrollable without animated transitions being required
