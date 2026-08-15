## 1. Extend the `.article-prose` contract in App.css

- [x] 1.1 Add rules for the TipTap table family (`.article-prose table.tiptap-table`,
      `thead`, `tbody`, `tr`, `th`, `td`) using the ink-tide surface,
      border, padding, and the same border-radius as `figure img`.
- [x] 1.2 Add rules for task lists (`.article-prose ul[data-type="taskList"]`,
      `li[data-type="taskItem"]`, `li[data-type="taskItem"][data-checked="true"]`,
      and the inline checkbox) so the list keeps the existing list spacing
      and the checked item's text drops to the muted ink color.
- [x] 1.3 Add rules for the inline marks the editor emits (`.article-prose u`,
      `.article-prose s`, `.article-prose mark`, `.article-prose sub`,
      `.article-prose sup`) and inherit the line-height so subscript and
      superscript do not change the paragraph rhythm.
- [x] 1.4 Add rules for standalone images and YouTube embeds
      (`.article-prose img`, `.article-prose iframe`,
      `.article-prose .iframe-wrapper`) so they stay within the 68ch
      measure, keep their rounded corners, and inherit the figure
      vertical rhythm.
- [x] 1.5 Confirm the existing `pre` / `code` / `pre.code-block` / `pre code.hljs`
      rules still apply by re-reading the post-build CSS and grepping
      for the rule selectors. Adjust only if `lightningcss` further
      fragments a rule.

## 2. Strengthen the post-page test

- [x] 2.1 In `apps/ducth-dev-website/src/pages/pages.test.tsx`, extend the
      "renders the post reading surface" scenario with a TipTap document
      that includes a table, a task list, an inline `<u>` and `<mark>`,
      and an `<img class="rounded max-w-full h-auto">`. Assert that the
      closest `.article-prose` wrapper exists around all of them and
      that the wrapper does not carry the Tailwind Typography `prose`
      class.
- [x] 2.2 (test runner blocked by sandbox spawn EPERM; vitest must be run locally) Run `pnpm test` (or the project's documented vitest command) in
      `apps/ducth-dev-website` and confirm the new assertions pass.

## 3. Sync the design notes

- [x] 3.1 In `apps/ducth-dev-website/design/02-typography.md`, replace the
      "`Article | prose prose-lg | max-w-none | Typography plugin styles TipTap HTML`"
      row with one that describes the canonical `.article-prose` contract
      and notes that Tailwind Typography is intentionally not used.
- [x] 3.2 In `apps/ducth-dev-website/design/08-components.md`, update the
      Article-detail bullet that currently says "Semantic `<article>` uses
      `prose prose-lg max-w-none`" to reference the `.article-prose`
      contract instead.

## 4. Build, verify, and archive

- [ ] 4.1 Run `cd apps/ducth-dev-website && pnpm build` (or
      `node node_modules/@rsbuild/core/bin/rsbuild.js build` if `pnpm` is
      blocked by the sandbox) and confirm the new CSS file contains the
      new `.article-prose` selectors.
- [ ] 4.2 Run `openspec verify fix-public-blog-prose-styling` and resolve
      any `CRITICAL` finding. `WARNING` findings are acceptable only if
      the rationale is recorded in `design.md`.
- [ ] 4.3 Run `openspec sync-specs fix-public-blog-prose-styling` and
      `openspec archive fix-public-blog-prose-styling` so the delta
      specs land in `openspec/specs/website-design-system/spec.md` and
      `openspec/specs/website-reader-experience/spec.md`.