/**
 * Resolve a media URL from a relative path and a media base URL.
 *
 * - If `path` is already an absolute URL (starts with `http://` or `https://`)
 *   it is returned verbatim.
 * - Otherwise, `mediaBaseUrl` and `path` are joined with exactly one slash,
 *   regardless of whether `mediaBaseUrl` ends with `/` or `path` starts with
 *   `/`.
 *
 * @param path          The relative path or absolute URL.
 * @param mediaBaseUrl  The media base URL (e.g. "https://api.example.test/media").
 */
export function getMediaUrl(path: string, mediaBaseUrl: string): string {
  if (path.startsWith('http://') || path.startsWith('https://')) {
    return path;
  }
  const base = mediaBaseUrl.replace(/\/$/, '');
  const rel = path.replace(/^\//, '');
  return `${base}/${rel}`;
}
