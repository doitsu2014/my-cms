import { describe, it, expect } from 'vitest';
import { getMediaUrl } from './get-media-url';

describe('getMediaUrl — spec-mandated scenarios', () => {
  it('(a) joins a path without leading slash with exactly one slash', () => {
    expect(getMediaUrl('wwlkmlklf2-duc-tran-png.png', 'https://x.test/media')).toBe(
      'https://x.test/media/wwlkmlklf2-duc-tran-png.png',
    );
  });

  it('(b) does not double-slash when the path has a leading slash and the base has a trailing slash', () => {
    expect(getMediaUrl('/wwlkmlklf2-duc-tran-png.png', 'https://x.test/media/')).toBe(
      'https://x.test/media/wwlkmlklf2-duc-tran-png.png',
    );
  });

  it('(c) returns the absolute URL verbatim when the path starts with http:// or https://', () => {
    expect(
      getMediaUrl('https://cdn.example.test/x.png', 'https://x.test/media'),
    ).toBe('https://cdn.example.test/x.png');
  });
});

describe('getMediaUrl — edge cases', () => {
  it('handles path with leading slash and base without trailing slash', () => {
    expect(getMediaUrl('/x.png', 'https://x.test/media')).toBe(
      'https://x.test/media/x.png',
    );
  });

  it('handles path without leading slash and base with trailing slash', () => {
    expect(getMediaUrl('x.png', 'https://x.test/media/')).toBe(
      'https://x.test/media/x.png',
    );
  });

  it('returns http:// absolute URLs verbatim', () => {
    expect(getMediaUrl('http://cdn.example.test/x.png', 'https://x.test/media')).toBe(
      'http://cdn.example.test/x.png',
    );
  });
});
