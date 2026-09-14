import { describe, expect, it } from 'vitest';
import { getLocalizedPost } from './getLocalizedPost';
import type { BlogPost } from '../../types/content';

const post = {
  id: 'post-1',
  title: 'English',
  content: '<p>English content</p>',
  postTranslations: {
    nodes: [{ languageCode: 'vi', title: 'Tiếng Việt', content: '<pre><code class="language-mermaid">flowchart LR\nA-->B</code></pre>' }],
  },
} as BlogPost;

describe('getLocalizedPost', () => {
  it('uses a diagram-only translation only for the exact Mermaid language class', () => {
    expect(getLocalizedPost(post, 'vi').content).toContain('flowchart LR');
    const falsePositive = {
      ...post,
      postTranslations: { nodes: [{ languageCode: 'vi', content: '<pre><code class="language-mermaid-note">not a diagram</code></pre>' }] },
    } as BlogPost;
    expect(getLocalizedPost(falsePositive, 'vi').content).toBe('<p>English content</p>');
  });
});
