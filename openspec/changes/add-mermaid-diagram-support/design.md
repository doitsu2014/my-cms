## Context

This is a proposal for Mermaid authoring and reading in `apps/web` and `apps/ducth-dev-website`. Product scope is in `proposal.md`; no stakeholder approval or implementation is implied by artifact completion.

Current source evidence inspected on 2026-09-13:

| Boundary | Evidence and consequence |
| --- | --- |
| Admin forms | `apps/web/src/app/admin/blogs/blog-form.tsx` mounts `RichTextEditor` for base and translated content. That export aliases `TipTapEditor`; `rich-text-editor-wrapper.tsx` is an additional lazy entry. Both reach the same implementation. |
| Editable document | `.../rich-text-editor/tiptap-editor.tsx` disables StarterKit code blocks, installs CodeBlockLowlight, emits `editor.getHTML()`, loads changes via `setContent`, and previews through shared ArticleProse. Rendered presentation must never enter this editor state. |
| Interchange | `.../toolbar/toolbar.tsx` imports with marked and exports with Turndown. Current import removes boundary newlines and export always uses three backticks; Mermaid needs targeted round-trip handling. |
| Highlighting | Installed CodeBlockLowlight's `src/lowlight-plugin.ts` auto-detects unknown languages. Website `PostDetailPage.tsx` highlights every `pre code`. Both need an explicit Mermaid exclusion. Admin `src/init-highlight.ts` only exposes Highlight.js globally; no additional diagram integration belongs there. |
| Shared presentation | `packages/editor-prose/src/ArticleProse.tsx` is a single raw-HTML wrapper. Existing package/admin/reader smoke tests pin ordinary HTML. Canonical package spec currently prohibits non-React runtime dependencies; the delta explicitly changes that contract. |
| Reader lifecycle | `PostDetailPage.tsx` obtains localized HTML, then renders the package re-export in `src/components/posts/ArticleProse.tsx`. `src/index.server.tsx` uses Apollo SSR and `renderToString`; browser-only work must begin after hydration. |
| Localization | `src/lib/i18n/getLocalizedPost.ts` accepts translations only when they contain `<p>`. A diagram-only translation requires a narrow additional recognized-content case. |
| Packaging | Both apps use `file:../../packages/editor-prose` and have independent lockfiles; the package has its own lockfile. Both Dockerfiles use Node 20. Reinstall the local package in each consumer when dependencies change. |

Graph gate: `get_minimal_context(task="add-mermaid-diagram-support")` returned `ok`, 2,089 nodes, 20,472 edges, matching HEAD/build SHA `ebe8a1af046670118ab907907e7e034cf2d5893b`, updated 2026-08-23. Inspected `components-handle`, `RichTextEditorWrapper` flow (toolbar/auth boundary), TipTapEditor callees, qualified ArticleProse callers/tests, and PostDetailPage imports. ArticleProse tests and admin preview were found; some imports/re-exports remained unresolved. Targeted source search supplemented graph coverage, identifying localization and the public re-export. Existing tests are smoke coverage, not Mermaid security or lifecycle coverage. `update_plan` is not exposed in this session; artifact dependency/status checks track the sequence instead.

## Goals / Non-Goals

**Goals:** source-based insertion and editing; Article Preview and public rendering; HTML/Markdown round-trip; safe local failure; localization, SSR, accessibility, and ordinary-code compatibility.

**Non-Goals:** live editor node views, visual diagram editing, API changes, schema or data migration, SVG persistence/export, AI work, remote rendering, new publishing validation, and unrelated HTML sanitization changes. The existing article-HTML trust boundary is unchanged; the new diagram path must not weaken it. No `apps/api` work is planned, so API architecture mapping and SeaORM generation are not required.

## Decisions

### D1. Use an exact local Mermaid dependency compatible with current builds

Add `mermaid: "11.17.2"` to `packages/editor-prose/package.json`; update its lockfile and both consumer lockfiles through pnpm installation. Import the package entry with dynamic `import('mermaid')` inside a browser-only helper, never through a CDN or the SSR import path. This official release provides an ESM entry and types. Verify its resolved transitive engine requirements and advisory status during installation; do not silently upgrade the application runtime to resolve a dependency conflict. The [tagged manifest](https://raw.githubusercontent.com/mermaid-js/mermaid/mermaid@11.17.2/packages/mermaid/package.json) and [release](https://github.com/mermaid-js/mermaid/releases/tag/mermaid@11.17.2) establish the baseline. Actual Node 20 install/build verification remains an implementation check.

Alternatives: Mermaid 12 requires newer Node/browser baselines and changes layout defaults, as stated in its [release notes](https://github.com/mermaid-js/mermaid/releases/tag/mermaid@12.0.0); adopting it would broaden scope. A React wrapper adds lifecycle policy without solving persistence. Mermaid CLI or remote services introduce server/asset workflows. Choose the official browser package and share one integration across both consumers.

### D2. Preserve the existing code-block model and source format

Use TipTap's existing `codeBlock` node with `language: 'mermaid'`, serialized as `<pre><code class="language-mermaid">escaped source</code></pre>`; preserve compatible additional classes such as `code-block`. Recognition requires the exact `language-mermaid` class token on a code child of a pre element. Never infer Mermaid from ordinary text or a substring class match.

Add a named Mermaid toolbar button: within a code block set its language without losing text; otherwise insert a block with a small valid flowchart and position focus in its source. Existing undo/redo and read-only behavior apply. Register a plain-text Mermaid language with the existing lowlight instance to prevent its unknown-language auto-detection; keep ordinary languages unchanged. Preview uses the existing Article Preview modal; no rendering inside the editable document.

Extract small Markdown conversion helpers from `toolbar.tsx` if needed for meaningful tests. Preserve Mermaid boundary whitespace instead of the current code-block newline cleanup. For Mermaid export choose a fence longer than any backtick run in the source, preserve text via `textContent`, and compensate for the one structural newline added by fenced Markdown serialization/import. Normalize line endings to LF only; retain all meaningful source whitespace. HTML attributes and entities may normalize, but decoded diagram source must survive. Test HTML-edit apply, editor serialization/reload, and actual toolbar copy/import, not just an isolated string converter.

A custom node/React node view was rejected because it changes editor schema and round-trip behavior without being needed for the approved preview workflow. No stored SVG or Markdown conversion of all posts is necessary. Existing content strings, API calls, concurrency checks, transactions, authorization, and database fields remain unchanged.

### D3. Share scoped enhancement after hydration

Extend ArticleProse with a root ref and effect. Keep its SSR and first client render exactly as supplied HTML. If no recognized blocks exist, do not call the runtime loader. `src/mermaid-renderer.ts` owns the cached loader and rendering policy; `src/mermaid-enhancement.ts` owns per-root discovery, temporary DOM, and teardown. Public exports remain compatible.

For each recognized block capture source using `textContent`, then add a sibling presentation container scoped inside the existing article root. Keep readable source during loading. After success, show the generated SVG as an inert SVG image via a Blob URL and expose the original pre/code in a native `details`/`summary` source disclosure. Do not inject the returned SVG as interactive article HTML or call Mermaid's `bindFunctions`. The image has an accessible label and the disclosure provides the full text alternative. Blob URLs belong only to this mounted presentation and are revoked on replacement/unmount. Mermaid's render-time temporary container is outside the editable document and cleaned in `finally`.

State per block is source/loading → rendered or fallback. A new root content generation cancels queued work and invalidates pending results; check generation and element connectivity before committing any result. Use unique generated render IDs across instances, serialize render jobs through a module queue because Mermaid has shared configuration/temporary DOM, and yield between blocks. Effects must be idempotent under React StrictMode, same-HTML rerenders, multiple previews, and route/locale changes. Cleanup removes only owned presentation nodes and restores owned source wrappers; it must not rewrite newer HTML or interfere with Highlight.js.

Failures return categorized outcomes (`empty`, `unsupported`, `syntax`, `limit`, `load`, `render`) to local status text. Catch rejected imports without poisoning the loader cache, so a later mount can retry. A 10-second loading deadline changes UI to source fallback and discards that attempt's late result; it is not a synchronous render cancellation guarantee. Corrected source or reopening preview retries; no new Save/Publish gate or retry loop is introduced.

Alternative SSR SVG rendering would require a browser-like renderer and create hydration/operational complexity. Duplicating effects in the apps risks different safety behavior. Inert image presentation trades diagram text selection for shared source access and prevents interactive SVG behavior; verify deployed CSP permits locally generated `blob:` images before release.

### D4. Fix application-controlled security and rendering limits

Initialize the shared renderer with `startOnLoad: false`, `securityLevel: 'strict'`, `htmlLabels: false`, `suppressErrorRendering: true`, `maxTextSize: 50000`, `maxEdges: 500`, and a fixed light `neutral` theme. Preserve Mermaid's default secure keys and add all application-selected settings, including `htmlLabels`, `theme`, `themeCSS`, `themeVariables`, `dompurifyConfig`, and layout/diagram settings, to the protected configuration. The [configuration schema](https://mermaid.js.org/config/schema-docs/config.html) documents protected settings and limit controls; [usage guidance](https://mermaid.js.org/config/usage.html) documents strict mode and manual rendering. Verify available keys against the pinned package types, since live docs describe newer releases.

Do not accept author configuration in this first release: reject YAML frontmatter and Mermaid `%%{...}%%` directives before rendering, retaining safe source. This deliberately excludes presentation overrides as well as security overrides. Labels with ordinary HTML-like text are encoded by strict mode; executable callbacks and link binding remain disabled. Do not register remote icon loaders or external rendering services. Use the maintained library's sanitization path, never raw source `innerHTML`, and do not loosen its sanitizer. The generated image has no interactive links. Keep a visible fixed light canvas behind the neutral diagram in both app themes to avoid global theme reinitialization races.

Check source length before import/render and let Mermaid enforce graph edge limits. Rendering is serial and only active roots commit output. There is no honest hard timeout for synchronous browser layout: a promise timeout cannot interrupt blocked JavaScript. Keep this limitation in the risk register, exercise maximum-size fixtures, and investigate a worker/isolated renderer separately if real measurements require it. Do not log source, full parser exceptions, or SVG; local errors use sanitized category text. No new remote telemetry pipeline is needed. Verification records categories, chunk loading, and failure counts without article text.

### D5. Integrate reader localization and presentation affordances

Change the `PostDetailPage.tsx` highlighting selector/filter to exclude the exact Mermaid marker, including source disclosures; all other code retains existing highlighting. Update `getLocalizedPost.ts` to recognize canonical Mermaid pre/code as content in addition to its current paragraph criterion, using an SSR-safe helper that recognizes a pre/code language class rather than matching text inside unrelated markup. Retain existing empty/missing fallback and title/preview selection. Add unit cases for diagram-only translations and class-token false positives.

Keep styles in `packages/editor-prose/src/article-prose.css`: local diagram canvas, responsive image sizing, source/status spacing, local overflow, visible summary focus. Do not globally restyle article images or pre blocks. Use neutral canvas colors that contrast with generated labels in both themes. No motion is required. Add an optional presentation-labels prop with English defaults for loading/error/source/image labels; the website supplies English/Vietnamese strings based on `currentLang`, while admin uses defaults. It does not alter persisted content. The existing fixed Article Preview overlay should expose dialog semantics and retain/restore keyboard focus for preview interaction; limit any accessibility adjustment to this touched preview path.

No separate Product Designer brief was supplied. The feasible UX is the current toolbar plus Article Preview and native source disclosure. PD review at implementation may refine spacing, wording, and focus behavior within this scope; a live canvas or new authoring workflow requires returning to PO/SA.

## Risks / Trade-offs

| Risk | Mitigation / owner |
| --- | --- |
| Source mutation or Markdown fence loss | Source remains editor state; round-trip tests with whitespace/entities/backticks; SE. |
| Mermaid or transitive security/engine changes | Exact Mermaid version and committed lockfiles; advisory and Node 20 build checks before merge; SE. |
| CSS/config injection or interactive SVG | Reject configuration directives, strict library sanitization, protected config, inert image output, hostile-input real-browser checks; SE. |
| Large diagrams monopolize the main thread | 50k-character/500-edge limits, serialized work and yielding; record browser measurements and avoid claiming a hard execution deadline; SE. |
| Hydration races and stale Blob URLs | Server source only, generation guards, cleanup and actual SSR/hydration tests; SE. |
| Independent file dependencies become stale | Regenerate three lockfiles and reinstall both consumers, then build both; SE. |
| Deployed CSP or stale assets prevent images/runtime loads | Source fallback, deployed-policy check, cache-safe asset rollout; RE after implementation. |
| Diagram-only locale eligibility affects other consumers | Narrow marker recognition and existing localization regression cases; SE. |

## Migration Plan

No database migration, backfill, API version, or generated SeaORM change. Existing marked blocks render immediately without resave. At implementation, update local package dependencies and reinstall both apps. Build the website's client and server bundles and the admin bundle. Retain fingerprinted old assets during normal rollout so active clients can still load their matching diagram chunks. Deploy the website reader before or alongside admin authoring. RE receives dependency, bundle, CSP, fallback, and browser-verification evidence; production deployment is outside this proposal's authorization.

Rollback by restoring the previous app/package artifacts and lockfiles. Newly written diagrams remain ordinary language-marked code on older clients. No content rewrite is needed. If runtime loading or policy checks fail, source fallback remains readable while the release is corrected or reverted.

## Verification and Traceability

| Proposal outcome | Requirements / design | Tasks / verification |
| --- | --- | --- |
| 1 Author + preview + reopen | Authoring/source requirements; D2 | 2.1–2.3; admin Mermaid tests and manual editor save/reopen |
| 2 HTML/Markdown round-trip | Source requirement; D2 | 2.2–2.3; Unicode/entities/whitespace/fence fixtures |
| 3 Both consumers + locale | Rendering/reader requirements; D3/D5 | 1.2, 3.1–3.2; shared and reader tests |
| 4 Isolated failure/recovery | Failure/security requirements; D3/D4 | 1.1–1.3, 4.1; async/security/real-browser cases |
| 5 Existing prose/highlighting | Modified shared/reader contracts; D2/D5 | 2.1, 3.1; existing smoke fixtures and ordinary code assertions |
| 6 Accessible/responsive | Presentation requirement; D3/D5 | 3.2, 4.1; keyboard, theme and 360px checks |
| 7 SSR + no eager runtime | Shared lazy enhancement contract; D1/D3 | 1.2, 3.2, 4.1–4.2; SSR/hydration and bundle/network inspection |

Tests use Vitest/Testing Library already installed. Stub the renderer for race/failure orchestration; include real pinned-library parser and browser rendering checks so mocks cannot conceal integration/security regressions. Do not assert exact SVG path geometry. Full repository gates are implementation obligations in tasks.md, not checks performed for this documentation-only proposal.

## Open Questions and Readiness

No unresolved product decision blocks the proposed artifacts. Assumptions: website means `apps/ducth-dev-website`; preview means Article Preview; author configuration is unsupported; fixed neutral diagram canvas is acceptable. Dependency installation, actual browser/CSP verification, and PD implementation review remain explicit SE/RE prerequisites, not completed evidence. All tasks must remain unchecked until implementation verification passes. Next primary owner: Software Engineer when the user requests implementation; this proposal itself authorizes no product code or deployment.

## D6. Keep translation-job execution independent from editor refresh

`POST /posts/{post_id}/translate/background` already persists a translation job and runs it through `tokio::spawn`; no browser polling is required for the job to continue. The admin form currently polls `GET /posts/{post_id}/translate/jobs` and calls `reloadPostData()` when the active list becomes empty. That reload replaces `originalContent` and translation values, which rehydrates TipTap and can reset a rendered Article Preview.

Remove the recurring job-status effect and the immediate `checkActiveJobs()` calls after translation/retranslation starts. Retain the existing POST flow, success toast, and server-side job lifecycle. The form does not infer completion or reload automatically. Add an explicit Refresh content control near translation actions; it invokes the existing post read once. If React Hook Form is dirty, require the author to keep their current draft or discard it before that refresh can replace values. Do not call the jobs-list endpoint from the edit form after this change.

This deliberately favors draft safety and predictable previews over live translation progress. A future status feature can use a push mechanism or an isolated status surface that never rehydrates the form.
