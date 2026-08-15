## 1. Scaffold the shared package

- [x] 1.1 Create `packages/editor-prose/` with `package.json`,
      `tsconfig.json`, `README.md`, `src/index.ts`,
      `src/ArticleProse.tsx`, `src/article-prose.css`, and a vitest
      config that consumes the same Toolchain as the existing apps.
- [x] 1.2 Set `package.json` `exports` to `{ ".": "./src/index.ts",
      "./styles.css": "./src/article-prose.css" }`, declare React 18+
      as a peer dependency, and pin `"type": "module"`.
- [x] 1.3 Move the 45 `.article-prose` selectors from
      `apps/ducth-dev-website/src/App.css` into
      `packages/editor-prose/src/article-prose.css` verbatim, in the
      same order and with the same comments.
- [x] 1.4 Add a Vitest smoke test
      (`packages/editor-prose/src/__tests__/article-prose.test.tsx`)
      that mounts the fixture document (heading, paragraph, list,
      blockquote, code block, task list item, inline marks, table)
      and asserts each element is present in the rendered DOM under
      `.article-prose`. Confirm `pnpm exec vitest run` is green from
      the package directory.

## 2. Switch the reader app to the package

- [x] 2.1 Add `"editor-prose": "file:../../packages/editor-prose"` to
      `apps/ducth-dev-website/package.json` `dependencies` and run
      `pnpm install` from the app directory.
- [x] 2.2 Update `apps/ducth-dev-website/src/App.tsx` (or the entry
      CSS file it loads) to import `editor-prose/styles.css` once at
      the top of the stylesheet chain, after Tailwind layers.
- [x] 2.3 Replace `apps/ducth-dev-website/src/components/posts/ArticleProse.tsx`
      with a re-export of the shared component so existing imports
      (`PostDetailPage.tsx`, `pages.test.tsx`, etc.) keep resolving
      without churn.
- [x] 2.4 Delete the `.article-prose *` rule block from
      `apps/ducth-dev-website/src/App.css`. Run
      `git grep -n "article-prose" apps/ducth-dev-website/src/App.css`
      and confirm zero matches remain.
- [x] 2.5 Run `pnpm test`, `pnpm typecheck`, `pnpm lint`, and
      `pnpm build` in `apps/ducth-dev-website`. All must stay green
      with zero new warnings or lints.

## 3. Switch the admin preview to the package

- [x] 3.1 Add `"editor-prose": "file:../../packages/editor-prose"` to
      `apps/web/package.json` `dependencies` and run `pnpm install`.
- [x] 3.2 Update
      `apps/web/src/app/admin/components/inputs/rich-text-editor/tiptap-editor.tsx`
      so the "Article Preview" modal body renders
      `<ArticleProse html={editor.getHTML()} />` from the package
      instead of the inline `tiptap-preview` div with
      `dangerouslySetInnerHTML`. Keep the toolbar, fullscreen toggle,
      and HTML edit modal untouched.
- [x] 3.3 Import `editor-prose/styles.css` once in
      `apps/web/src/app/admin/components/inputs/rich-text-editor/tiptap-editor.tsx`
      (or a co-located `.css` file) so the package styles are
      available inside the preview modal.
- [x] 3.4 Delete the `.tiptap-preview *` rule block from
      `apps/web/src/app/admin/components/inputs/rich-text-editor/tiptap-editor.css`.
      Confirm zero matches with
      `git grep -n "tiptap-preview" apps/web/src/app/admin/components/inputs/rich-text-editor/tiptap-editor.css`.
- [ ] 3.5 Run `pnpm test`, `pnpm typecheck`, `pnpm lint`, and
      `pnpm build` in `apps/web`. All must stay green with zero new
      warnings or lints.

## 4. Cross-app verification

- [x] 4.1 Add a Vitest smoke test in
      `apps/ducth-dev-website/src/pages/__tests__/prose-package.test.tsx`
      that imports `<ArticleProse>` from `editor-prose` (not from the
      local wrapper) and renders the same fixture document. Confirm
      `pnpm test` is green.
- [x] 4.2 Add a Vitest smoke test in `apps/web/src/__tests__/editor-prose.test.tsx`
      that imports `<ArticleProse>` from `editor-prose`, renders the
      same fixture document, and asserts the rendered DOM matches the
      reader's output for the same fixture (compare serialized HTML,
      not snapshot strings).
- [x] 4.3 Confirm the existing reader test
      `apps/ducth-dev-website/src/pages/pages.test.tsx > wraps every
      TipTap element in the .article-prose contract without Tailwind
      Typography` still passes after the move.
- [x] 4.4 Run the full repository verification gate:
      `cargo check --workspace` from `apps/api/`,
      `pnpm --dir apps/ducth-dev-website test build`,
      `pnpm --dir apps/web test build`. Resolve every new warning or
      lint.

## 5. Documentation and OpenSpec hygiene

- [x] 5.1 Update `apps/ducth-dev-website/design/02-typography.md` and
      `08-components.md` so the prose row reflects that the contract
      lives in the package, not in the app's local CSS.
- [x] 5.2 Run `openspec validate share-editor-prose-design-package`
      and resolve every `CRITICAL` finding before requesting review.
- [x] 5.3 Run `openspec status --change share-editor-prose-design-package --json`
      and confirm `isComplete: true` with every artifact `done`.
