## 1. Define the metadata policy and pure resolver

- [x] 1.1 [website-and-admin-metadata] Create a typed public metadata module under `apps/ducth-dev-website/src/` that resolves a route/state plus localized public content and `SITE_CONFIG` into title, plain-text bounded description, canonical URL, document language, robots, Open Graph, and Twitter values. Include supported-locale normalization and safe default/error/not-found profiles. Test first with unit cases for EN/VI static routes, article/category routes, absent optional fields, query/hash removal, unpublished/missing content, and markup-significant text.
- [x] 1.2 [website-and-admin-metadata] Add a public head serializer/DOM adapter that renders only the managed title/meta/link tags from a resolved profile, HTML-escapes SSR values, and replaces (rather than duplicates) managed browser tags. Add focused tests for escaping and one-of-each-tag behavior.
- [x] 1.3 [website-and-admin-metadata] Define the administration route-metadata resolver under `apps/web/src/` with a meaningful product/admin title for login, dashboard, list, create, and edit route families and an invariant `noindex, nofollow` robots directive. Test route matching and fallback behavior before integration.

## 2. Integrate public metadata with SSR and browser navigation

- [x] 2.1 [website-and-admin-metadata] Extend `apps/ducth-dev-website/src/index.server.tsx` and the public route/content composition so SSR can return the public metadata profile after Apollo data loading, without altering GraphQL/API contracts or exposing unpublished content. Test representative localized post, category, static, missing, and failure-state server renders.
- [x] 2.2 [website-and-admin-metadata] Update `apps/ducth-dev-website/src/server.prod.mjs` and the production HTML template integration to inject the resolved metadata into `<head>` before the response is sent, alongside existing runtime config and Apollo state. Add SSR-output tests that parse the head and assert title, description, canonical, language, robots, Open Graph, and Twitter values occur exactly once.
- [x] 2.3 [website-and-admin-metadata] Wire the same public metadata profile into client route transitions in `apps/ducth-dev-website/src/AppContent.tsx` and/or the approved metadata boundary, including data-dependent post/category resolution. Test navigation between representative routes to prove stale values are replaced and no managed tag is duplicated.
- [x] 2.4 [website-and-admin-metadata] Update public page/content integration (`src/pages/` and supporting localization/media helpers only as needed) so localized post/category names and preview text feed the resolver, optional eligible thumbnails feed social images, and raw article HTML never becomes a description. Add regression cases for translation/fallback and missing-thumbnail behavior.

## 3. Integrate private admin metadata

- [x] 3.1 [website-and-admin-metadata] Update `apps/web/rsbuild.config.ts` (or its supported HTML-template configuration) so the built admin document has a product/admin default title and contains one `robots` directive with `noindex, nofollow` before the React application mounts.
- [x] 3.2 [website-and-admin-metadata] Integrate the central admin route-metadata resolver at the `apps/web/src/App.tsx` router boundary so login, dashboard, list, create, and edit navigations update the title while preserving a single private robots directive. Add representative route/navigation tests and assert that public canonical/Open Graph/Twitter metadata is absent.

## 4. Verify and document the policy

- [x] 4.1 [website-and-admin-metadata] Document the required public runtime metadata inputs and the public-vs-admin indexability policy in the relevant application configuration documentation, including the fallback behavior and the fact that CMS-editable SEO overrides are out of scope.
- [x] 4.2 [website-and-admin-metadata] Run `pnpm --dir apps/ducth-dev-website test`, `pnpm --dir apps/ducth-dev-website build`, `pnpm --dir apps/web test`, and `pnpm --dir apps/web build`; inspect representative production SSR response heads and admin build output. Record results and any approved title-template/social-card decisions in the implementation handoff.
