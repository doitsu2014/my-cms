import { describe, expect, it } from 'vitest';
import { htmlToMarkdown, markdownToEditorHtml } from './toolbar/toolbar';

describe('Mermaid Markdown interchange', () => {
  it('retains Mermaid source, language, Unicode, entities, and backtick runs', async () => {
    const source = 'flowchart LR\n  A[Hà Nội &amp; `label`] --> B[&lt;done&gt;]\n  note right of B: ````';
    const html = `<pre><code class="language-mermaid">${source}</code></pre>`;
    const markdown = htmlToMarkdown(html);

    expect(markdown).toContain('`````mermaid');
    expect(markdown).toContain('Hà Nội & `label`');
    await expect(markdownToEditorHtml(markdown)).resolves.toContain('class="language-mermaid"');
    await expect(markdownToEditorHtml(markdown)).resolves.toContain('Hà Nội &amp; `label`');
  });
});
