/**
 * Escape a value for safe inline JSON inside an HTML `<script>` tag.
 *
 * The seven characters that break inline JSON in HTML are:
 *   <, >, &, ', ", U+2028, U+2029
 *
 * After JSON.stringify, we replace the unsafe characters with their JSON-safe
 * Unicode escape sequences:
 *   <  → \u003c
 *   >  → \u003e
 *   &  → \u0026
 *   '  → \u0027
 *   "  → handled natively by JSON.stringify
 *   U+2028 → \u2028
 *   U+2029 → \u2029
 *
 * The output is a string that:
 *   1. Round-trips through JSON.parse to the original value.
 *   2. Does not contain the literal substring `</script>` (which would
 *      terminate the surrounding `<script>` element prematurely).
 *   3. Does not break a strict JSON parser on the client (notably U+2028 and
 *      U+2029 which were never valid in JSON until ES2019).
 *
 * Per RFC 8259 §7 and OWASP JSON-in-HTML guidance.
 */
export function escapeJsonForScript(value: unknown): string {
  const json = JSON.stringify(value) ?? 'null';

  // The order matters: replace `</script>` first to prevent the `</` from
  // being clobbered by the `<` replacement below.
  return json
    .replace(/</g, '\\u003c')
    .replace(/>/g, '\\u003e')
    .replace(/&/g, '\\u0026')
    .replace(/\u2028/g, '\\u2028')
    .replace(/\u2029/g, '\\u2029');
}
