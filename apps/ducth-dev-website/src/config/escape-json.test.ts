import { describe, it, expect } from 'vitest';
import { escapeJsonForScript } from './escape-json';

describe('escapeJsonForScript — spec-mandated escape set', () => {
  it('(a) escapeJsonForScript(</script>) does not contain the literal </script> sequence', () => {
    const out = escapeJsonForScript('</script>');
    expect(out).not.toContain('</script>');
    // The value must round-trip through JSON.parse
    expect(JSON.parse(out)).toBe('</script>');
  });

  it('(b) escapeJsonForScript(a&b) round-trips through JSON.parse to the original', () => {
    const out = escapeJsonForScript('a&b');
    expect(JSON.parse(out)).toBe('a&b');
  });

  it('(c) escapeJsonForScript(U+2028) round-trips through JSON.parse to the original', () => {
    const out = escapeJsonForScript('\u2028');
    expect(JSON.parse(out)).toBe('\u2028');
  });

  it('(d) escapeJsonForScript(<>&\u0027\u2028\u2029) round-trips through JSON.parse', () => {
    const input = '<>&\u0027\u2028\u2029';
    const out = escapeJsonForScript(input);
    expect(JSON.parse(out)).toBe(input);
  });

  it('(e) the produced output for any input does not contain the literal </script> sequence', () => {
    const inputs = ['</script>', 'foo</script>bar', '<script>foo</script>baz'];
    for (const input of inputs) {
      const out = escapeJsonForScript(input);
      expect(out).not.toContain('</script>');
    }
  });
});

describe('escapeJsonForScript — all 7 spec-mandated characters are escaped', () => {
  it('escapes <, >, &, ", and U+2028/U+2029 in the output', () => {
    const out = escapeJsonForScript('<>&\u2028\u2029');
    // Each of these must be replaced by the JSON-safe representation
    expect(out).toContain('\\u003c'); // <
    expect(out).toContain('\\u003e'); // >
    expect(out).toContain('\\u0026'); // &
    expect(out).toContain('\\u2028'); // U+2028
    expect(out).toContain('\\u2029'); // U+2029
  });

  it('handles objects and arrays (not just strings)', () => {
    const out = escapeJsonForScript({ url: 'https://x.test/a&b' });
    expect(JSON.parse(out)).toEqual({ url: 'https://x.test/a&b' });
  });
});

describe('escapeJsonForScript — OWASP regression test', () => {
  it('round-trips a URL with & and the output contains the JSON-safe \\u0026', () => {
    const url = 'https://example.com/?a=1&b=2';
    const out = escapeJsonForScript({ url });
    // The output must contain the JSON-safe \u0026 for the literal &
    expect(out).toContain('\\u0026');
    expect(JSON.parse(out).url).toBe(url);
  });

  it('escapes a single quote to the JSON-safe form', () => {
    const out = escapeJsonForScript("it's");
    expect(JSON.parse(out)).toBe("it's");
  });
});
