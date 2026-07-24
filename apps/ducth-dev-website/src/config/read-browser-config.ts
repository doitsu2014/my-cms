import type { RuntimeConfig } from './runtime-config';

/**
 * Read the runtime configuration from the `<script id="app-config">` element
 * injected into the SSR HTML.
 *
 * The SSR handler writes the resolved config into a `<script id="app-config"
 * type="application/json">` tag immediately before `</head>`. This function
 * reads the tag's text content and parses it with `JSON.parse`.
 *
 * - Throws an Error whose message contains `app-config` when the script tag is
 *   absent.
 * - Throws the underlying `SyntaxError` when the script tag's content is
 *   malformed JSON.
 *
 * @param doc  The document to read from. Defaults to the global `document`.
 *              Exposed for testing.
 */
export function readBrowserConfig(doc: Document = document): RuntimeConfig {
  const script = doc.getElementById('app-config');
  if (!script || !script.textContent) {
    throw new Error(
      'readBrowserConfig: <script id="app-config" type="application/json"> is missing from the document',
    );
  }
  return JSON.parse(script.textContent) as RuntimeConfig;
}
