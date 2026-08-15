## ADDED Requirements

### Requirement: Post article body uses the shared `ArticleProse` component

The post page SHALL render the localized TipTap HTML through the
`<ArticleProse>` component imported from the shared `editor-prose`
package. The page SHALL NOT mount its own inline-styled `.article-prose`
div with `dangerouslySetInnerHTML`; the wrapping and the styling rule
set both come from the package.

#### Scenario: Post page renders through the shared component

- **WHEN** a post page renders at `/:lang/posts/:slug`
- **THEN** the article body element is the `<div class="article-prose">`
  mounted by `<ArticleProse>` from `editor-prose`
- **AND** no other wrapper element wraps the localized `post.content`
  HTML between the featured image and the share row

#### Scenario: Highlight.js hooks still fire

- **WHEN** the post page renders
- **THEN** the existing `useEffect` that calls `hljs.highlightElement` on
  every `<pre><code>` inside the article body still fires (the shared
  component does not swallow the ref the page uses to traverse the DOM)
