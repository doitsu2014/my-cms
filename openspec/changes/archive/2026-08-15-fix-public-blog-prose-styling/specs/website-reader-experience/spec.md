## MODIFIED Requirements

### Requirement: Post reading experience

The post page at `/:lang/posts/:slug` SHALL render an editorial header
(category pill, date, optional reading time when data is available, title,
deck, byline), a featured image at 21:9, an article body constrained to 68ch,
a share row, and a related posts section. The article body SHALL use the
`.article-prose` contract and SHALL wrap the full TipTap HTML document so the
contract applies to every node the editor can produce. The Highlight.js code
highlighting integration SHALL be preserved.

#### Scenario: Article body stays within 68ch

- **WHEN** the post page renders
- **THEN** the article body's computed width is at most 68ch

#### Scenario: Featured image renders at 21:9

- **WHEN** the post has a featured image
- **THEN** the image renders at a 21:9 aspect ratio with a stable crop and
  the empty alt is avoided by using a localized alt derived from the title

#### Scenario: Code blocks remain highlighted

- **WHEN** the post body contains a TipTap code block
- **THEN** Highlight.js applies the existing syntax classes and the code
  block scrolls horizontally on overflow

#### Scenario: Article body carries the prose contract class

- **WHEN** the post page renders the article body
- **THEN** the immediate wrapper of the rendered TipTap HTML carries the
  `.article-prose` class and no Tailwind Typography `prose` utility