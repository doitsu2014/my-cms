# website-frontend Specification (delta)

## Purpose
Migrated public reader. Preserves the React 19 + Rsbuild + Express 5 SSR application formerly at `my-blogs-rsbuild/client_side/` so the reader runs as a standalone pnpm app adjacent to `apps/web/` and consumes the my-cms public GraphQL endpoint (`/graphql/immutable`). All hardcoded production URLs are replaced by a single server-controlled runtime configuration that is safely injected into the SSR HTML and consumed by the browser bundle.

## ADDED Requirements

### Requirement: Locale-prefixed routing is preserved

The reader SHALL expose the route shape previously defined at `my-blogs-rsbuild/client_side/src/AppContent.tsx:27-40`: `/`, `/:lang`, `/:lang/categories`, `/:lang/categories/:slug`, `/:lang/posts/:slug`, and `*` (catch-all). The root path `/` SHALL redirect to `/en`. Any unknown path SHALL redirect to `/en`. `:lang` SHALL be one of `en` or `vi`; unknown `:lang` values SHALL render the `en` content.

#### Scenario: Root redirects to /en
- **WHEN** a user requests `/`
- **THEN** the SSR handler returns HTTP 302 with `Location: /en`

#### Scenario: Unknown locale falls back to en content
- **WHEN** a user requests `/zz/posts/my-article`
- **THEN** the SSR handler returns HTTP 200 and the rendered HTML contains the article body translated from the default `en` source (no `postTranslations` match is returned to the client for that locale)

#### Scenario: Unknown route redirects to /en
- **WHEN** a user requests `/some/random/path`
- **THEN** the SSR handler returns HTTP 302 with `Location: /en`

### Requirement: SSR and hydration contract

The server entry `src/index.server.tsx` SHALL render the app tree to an HTML string using `renderToString` together with Apollo `getDataFromTree`, returning `{ html, apolloState }`. The client entry `src/index.client.tsx` SHALL hydrate the resulting HTML using `hydrateRoot` and SHALL restore the Apollo cache from `window.__APOLLO_STATE__`. The HTML template SHALL contain a `<!--app-content-->` placeholder that the SSR handler replaces with the rendered markup.

#### Scenario: SSR populates Apollo cache before stringifying
- **WHEN** the SSR handler receives `GET /en`
- **THEN** the server entry awaits `getDataFromTree(App)` so all GraphQL queries in the rendered tree resolve before `renderToString` runs
- **AND** the returned `apolloState` is the result of `client.extract()` on the same client used to render

#### Scenario: Client hydrates without warnings
- **WHEN** the browser receives the SSR HTML for `/en`
- **THEN** `hydrateRoot` attaches without a React hydration mismatch warning in the developer console for an unmodified build

### Requirement: Runtime configuration is owned by the server

The Express server SHALL be the single source of truth for runtime configuration. The server SHALL read the values listed below from `process.env` exactly once at startup. The server SHALL exit non-zero with a single log line naming the missing or invalid variable if any required value is absent or fails `new URL(value)` validation.

Required values:
- `WEBSITE_PUBLIC_GRAPHQL_API_URL`
- `WEBSITE_PUBLIC_MEDIA_BASE_URL`
- `WEBSITE_SITE_URL`
- `WEBSITE_SITE_NAME`
- `WEBSITE_DEFAULT_TITLE`
- `WEBSITE_DEFAULT_DESCRIPTION`

Optional values with documented defaults:
- `WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL` (defaults to `WEBSITE_PUBLIC_GRAPHQL_API_URL`)
- `WEBSITE_AVATAR_URL` (no default; when absent, the config payload omits `avatarUrl` and the reader renders no avatar)
- `WEBSITE_DEFAULT_LOCALE` (defaults to `"en"`)
- `WEBSITE_PORT` (defaults to `3001`)

#### Scenario: Missing required URL exits with a clear log
- **WHEN** the server is started with `WEBSITE_PUBLIC_GRAPHQL_API_URL` unset
- **THEN** the process exits with a non-zero status
- **AND** the last log line includes the string `WEBSITE_PUBLIC_GRAPHQL_API_URL is required`
- **AND** the process does not bind port 3001

#### Scenario: Malformed URL exits with a clear log
- **WHEN** the server is started with `WEBSITE_PUBLIC_GRAPHQL_API_URL=not-a-url`
- **THEN** the process exits with a non-zero status
- **AND** the last log line includes `WEBSITE_PUBLIC_GRAPHQL_API_URL` and the first 64 characters of the offending value

#### Scenario: Optional avatar falls back to no avatar
- **WHEN** the server is started without `WEBSITE_AVATAR_URL`
- **THEN** the process starts normally
- **AND** the rendered HTML does not include an `<img>` element whose `src` points at `undefined` or `null`

### Requirement: SSR HTML injection is safe against inline-JSON pitfalls

The SSR handler SHALL inject the resolved runtime configuration into every response as a single `<script id="app-config" type="application/json">…</script>` element placed immediately before the closing `</head>` tag. The JSON payload SHALL escape the seven characters that break inline JSON embedded in HTML: `<`, `>`, `&`, `'`, `"`, U+2028, U+2029 (per RFC 8259 §7 and OWASP JSON-in-HTML guidance). The client SHALL read the script tag's text content using `JSON.parse` and SHALL expose the result as `window.__APP_CONFIG__`. The script SHALL NOT use `type="text/javascript"` (which would execute) and SHALL NOT use unescaped string interpolation.

#### Scenario: Ampersand in avatar URL renders safely
- **WHEN** `WEBSITE_AVATAR_URL` contains `&` (for example `https://example.com/a?x=1&y=2`)
- **THEN** the rendered HTML contains `&amp;` in place of the bare `&` inside the script tag's text content
- **AND** the client parses the script tag successfully
- **AND** `window.__APP_CONFIG__.avatarUrl` is the original unescaped URL

#### Scenario: Closing script tag in a value cannot escape the script
- **WHEN** any resolved config value contains the literal text `</script>`
- **THEN** the rendered HTML escapes the `/` so the script element does not terminate prematurely
- **AND** `JSON.parse` on the client still returns the original value

#### Scenario: U+2028 line separator does not break JSON.parse
- **WHEN** any resolved config value contains U+2028
- **THEN** the rendered HTML escapes it so a strict JSON parser on the client succeeds

### Requirement: Browser access reads window.__APP_CONFIG__ only

The browser bundle SHALL NOT read `process.env` at runtime. `src/infrastructure/graphql/graphql-client.ts` SHALL build its `httpLink` URI from `window.__APP_CONFIG__.graphqlApiUrl`. Page components and helper functions SHALL resolve every external URL through helpers (`getMediaBaseUrl()`, `getMediaUrl(path)`) that read `window.__APP_CONFIG__`. The literal strings `https://my-cms-api.ducth.dev` and `https://my-blogs.ducth.dev` SHALL NOT appear anywhere under `apps/ducth-dev-website/src/`.

#### Scenario: GraphQL client uses injected URL
- **WHEN** the SSR handler injects `graphqlApiUrl=https://example.test/graphql`
- **THEN** the browser bundle issues Apollo queries to `https://example.test/graphql`
- **AND** no query is issued to any other origin

#### Scenario: Hardcoded strings are absent from src
- **WHEN** a developer runs `rg "https://my-cms-api.ducth.dev|https://my-blogs.ducth.dev" apps/ducth-dev-website/src/`
- **THEN** the command returns zero matches

### Requirement: Media URL helper joins exactly one slash

`getMediaUrl(path)` SHALL return `path` unchanged when `path` starts with `http://` or `https://`. Otherwise it SHALL return `${mediaBaseUrl}/${path}` where the join inserts exactly one `/` regardless of whether `mediaBaseUrl` ends with `/` or `path` starts with `/`. Page components SHALL call `getMediaUrl` for every thumbnail URL.

#### Scenario: Path without leading slash is joined with one slash
- **WHEN** `path` is `"wwlkmlklf2-duc-tran-png.png"` and `mediaBaseUrl` is `"https://x.test/media"`
- **THEN** `getMediaUrl` returns `"https://x.test/media/wwlkmlklf2-duc-tran-png.png"`

#### Scenario: Path with leading slash is not double-slashed
- **WHEN** `path` is `"/wwlkmlklf2-duc-tran-png.png"` and `mediaBaseUrl` is `"https://x.test/media/"`
- **THEN** `getMediaUrl` returns `"https://x.test/media/wwlkmlklf2-duc-tran-png.png"`

#### Scenario: Absolute URL is returned verbatim
- **WHEN** `path` is `"https://cdn.example.test/x.png"`
- **THEN** `getMediaUrl` returns `"https://cdn.example.test/x.png"` regardless of `mediaBaseUrl`

### Requirement: Public GraphQL queries

The reader SHALL issue GraphQL queries against the my-cms public endpoint `POST /graphql/immutable` (declared at `apps/api/src/bin/my-cms-api.rs:108-113`). The categories query SHALL filter `categoryType: { eq: Blog }` and `parentId: { is_null: "true" }`. The post-by-slug query SHALL filter `slug: { eq: $slug }`. Both queries SHALL request the related `categoryTranslations { nodes { languageCode displayName slug } }` and `postTranslations { nodes { languageCode title previewContent content } }` nodes respectively. The server entry SHALL run `getDataFromTree(App)` before `renderToString` so Apollo cache is fully populated server-side.

#### Scenario: Categories query shape
- **WHEN** the SSR handler renders `/:lang/categories`
- **THEN** the issued query includes `categories(filters: { categoryType: { eq: Blog }, parentId: { is_null: "true" } })`
- **AND** requests `categoryTranslations { nodes { languageCode displayName slug } }` on each category

#### Scenario: Post-by-slug query shape
- **WHEN** the SSR handler renders `/en/posts/my-article`
- **THEN** the issued query includes `posts(filters: { slug: { eq: "my-article" } })`
- **AND** requests `postTranslations { nodes { languageCode title previewContent content } }` on each post

### Requirement: Locale-based content selection

Page components SHALL read `:lang` from `useParams`. For each translatable field (title, previewContent, content), the component SHALL first look up `postTranslations.nodes.find(n => n.languageCode === lang)`; when no matching translation exists, the component SHALL fall back to the untranslated default field on the parent entity. The same logic applies to `categoryTranslations.nodes` for category display names.

#### Scenario: Vietnamese translation wins for title and content
- **WHEN** `currentLang === "vi"` and the current post has a `postTranslations` node with `languageCode: "vi"` and `title: "Bài viết mẫu"`
- **THEN** the rendered card and detail page show `"Bài viết mẫu"`
- **AND** the English fallback is not used

#### Scenario: Missing translation falls back to default
- **WHEN** `currentLang === "vi"` and the current post has no `postTranslations` node with `languageCode: "vi"`
- **THEN** the rendered card and detail page show the parent's English `title` and `content`

### Requirement: i18n bootstrap matches locale to URL

`src/i18n/i18n.ts` SHALL detect locale from `window.location.pathname` on the client and from the request URL on the server. When the first path segment is one of `en` or `vi`, that segment becomes the active locale. Otherwise the active locale falls back to `WEBSITE_DEFAULT_LOCALE` on the server and to `en` on the client. The active locale SHALL be applied to the i18next instance before the first render.

#### Scenario: Locale from URL is applied
- **WHEN** the SSR handler renders `/vi/posts/example`
- **THEN** the rendered HTML contains the Vietnamese translation of any `t('…')` string referenced by the rendered tree

### Requirement: SSR artifact shape

A successful `pnpm run build` SHALL produce `dist/client/index.html` and `dist/server/index.mjs`. `dist/server/index.mjs` SHALL export a default async function with signature `(url: string) => Promise<{ html: string; apolloState: object }>`.

#### Scenario: Build produces both bundles
- **WHEN** `pnpm --dir apps/ducth-dev-website build` completes
- **THEN** the file `apps/ducth-dev-website/dist/client/index.html` exists
- **AND** the file `apps/ducth-dev-website/dist/server/index.mjs` exists
- **AND** `node -e "import('./apps/ducth-dev-website/dist/server/index.mjs').then(m => console.log(typeof m.default))"` prints `function`

### Requirement: SSR errors do not leak details

The SSR request handler SHALL wrap every async block in `try`/`catch` so that the response is a generic HTML 500 page that omits the original error message. The server log line for an SSR error SHALL include the HTTP method, the URL path, and a generated correlation id; the log line SHALL NOT include the original error message, the request body, or any stack trace.

#### Scenario: Failed Apollo fetch yields generic 500
- **WHEN** an Apollo query during `getDataFromTree` rejects
- **THEN** the SSR handler returns HTTP 500 with a body that does not contain the error message
- **AND** the server log line for that request contains a UUID-shaped correlation id
- **AND** the server log line does not contain the string from the rejected error

#### Scenario: Missing template file yields generic 500
- **WHEN** `dist/client/index.html` is absent at startup or missing at request time
- **THEN** the SSR handler returns HTTP 500 with the same generic body
- **AND** the server log line does not contain the absolute path of the missing file
