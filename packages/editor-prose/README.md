# editor-prose

Shared React wrapper and CSS contract for rendered TipTap article content.

## What is in scope

`article-prose.css` styles the article wrapper, headings, paragraphs, links,
blockquotes, ordered and unordered lists, inline and block code, separators,
pull quotes, figures and images, TipTap tables, task lists, inline marks,
subscript/superscript, and YouTube embeds. It deliberately does not use
Tailwind Typography.

Mermaid diagrams are an optional browser-only enhancement of stored TipTap
source blocks. Write source as:

```html
<pre><code class="language-mermaid">flowchart LR
  A[Start] --> B[End]</code></pre>
```

Authors create or designate these blocks from the admin editor's Code Block
menu and use Article Preview to render them. Source stays authoritative: it is
available during loading and failures, is preserved in HTML and Markdown
interchange, and is never replaced by generated SVG. The renderer dynamically
loads Mermaid only when an article contains the exact `language-mermaid` class.
It rejects Mermaid directives and YAML frontmatter, uses strict rendering with
fixed application configuration, limits source to 50,000 characters and graph
edges to 500, and presents generated SVG through an inert local Blob image.

## Usage

```tsx
import { ArticleProse } from 'editor-prose';
import 'editor-prose/styles.css';

<ArticleProse html={post.content} />;
```

## Update flow

Edit `src/article-prose.css` to change the contract, then reinstall the local
`file:` dependency in both downstream apps (`apps/ducth-dev-website` and
`apps/web`). Run each app's tests and build before committing so both consumers
continue to render the same contract.

## Non-goals

This package does not use Tailwind Typography and does not inject CSS through
JavaScript. Consumers import the stylesheet once in their application entry
chain and supply the design tokens used by the contract.
