## MODIFIED Requirements

### Requirement: Post reading experience

The post page at `/:lang/posts/:slug` SHALL render an editorial header
(category pill, date, optional reading time when data is available, title,
deck, byline), a featured image, an article body constrained to 68ch, a
share row, and a related posts section. When a featured image exists, the
detail-page image SHALL render at the available content width using its
intrinsic aspect ratio, without stretching or cropping the source image.
When no featured image exists, the deterministic fallback artwork SHALL
preserve its requested layout aspect ratio. The article body SHALL use the
`.article-prose` contract and SHALL wrap the full TipTap HTML document so the
contract applies to every node the editor can produce. The Highlight.js code
highlighting integration SHALL be preserved.

#### Scenario: Article body stays within 68ch

- **WHEN** the post page renders
- **THEN** the article body's computed width is at most 68ch

#### Scenario: Featured image preserves its source aspect ratio

- **WHEN** the post has a featured image whose intrinsic ratio differs from 21:9
- **THEN** the image renders at the available content width
- **AND** its rendered height follows the source image's intrinsic aspect ratio
- **AND** the source image is not cropped by a cover-fit frame
- **AND** the empty alt is avoided by using a localized alt derived from the title

#### Scenario: Missing featured image retains deterministic fallback layout

- **WHEN** the post has no featured image
- **THEN** the detail page renders the deterministic slug-based fallback artwork
- **AND** the fallback preserves its requested layout aspect ratio

#### Scenario: Code blocks remain highlighted

- **WHEN** the post body contains a TipTap code block
- **THEN** Highlight.js applies the existing syntax classes and the code
  block scrolls horizontally on overflow

#### Scenario: Article body carries the prose contract class

- **WHEN** the post page renders the article body
- **THEN** the immediate wrapper of the rendered TipTap HTML carries the
  `.article-prose` class and no Tailwind Typography `prose` utility

