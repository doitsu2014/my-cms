import '@testing-library/jest-dom/vitest';
import { render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const { renderMermaid } = vi.hoisted(() => ({ renderMermaid: vi.fn() }));

vi.mock('../mermaid-renderer', () => ({
  renderMermaid,
}));

import { ArticleProse } from '../ArticleProse';

const diagram = '<pre><code class="language-mermaid">flowchart LR\nA --&gt; B</code></pre>';

afterEach(() => {
  renderMermaid.mockReset();
});

beforeEach(() => {
  vi.stubGlobal('URL', {
    createObjectURL: vi.fn(() => 'blob:diagram'),
    revokeObjectURL: vi.fn(),
  });
});

describe('ArticleProse Mermaid enhancement', () => {
  it('does not load a runtime for ordinary article HTML', () => {
    render(<ArticleProse html={'<p>Ordinary prose</p><pre><code class="language-javascript">const a = 1</code></pre>'} />);
    expect(renderMermaid).not.toHaveBeenCalled();
  });

  it('keeps Mermaid source while loading and renders an inert image after success', async () => {
    renderMermaid.mockResolvedValue({ ok: true, svg: '<svg xmlns="http://www.w3.org/2000/svg"></svg>' });
    const { container } = render(<ArticleProse html={diagram} />);

    expect(screen.getByText('Rendering diagram…')).toBeInTheDocument();
    expect(container.querySelector('code.language-mermaid')?.textContent).toBe('flowchart LR\nA --> B');

    await waitFor(() => expect(screen.getByRole('img', { name: /Mermaid diagram/i })).toBeInTheDocument());
    expect(screen.getByText('Diagram source (Mermaid)')).toBeInTheDocument();
    expect(container.querySelector('svg')).toBeNull();
  });

  it('keeps a local fallback for a failed diagram without changing ordinary code', async () => {
    renderMermaid.mockResolvedValue({ ok: false, category: 'syntax' });
    const { container } = render(<ArticleProse html={`${diagram}<pre><code class="language-mermaid-note">keep me as code</code></pre>`} />);

    await waitFor(() => expect(screen.getByRole('status')).toHaveTextContent('Diagram could not be rendered. Source remains available.'));
    expect(container.querySelector('code.language-mermaid-note')?.textContent).toBe('keep me as code');
    expect(screen.getByText('Diagram source (Mermaid)').closest('details')).toHaveProperty('open', true);
  });

  it('discards a delayed result after content replacement and revokes Blob URLs on unmount', async () => {
    let resolveFirst: (value: { ok: true; svg: string }) => void = () => undefined;
    renderMermaid.mockImplementationOnce(() => new Promise((resolve) => { resolveFirst = resolve; }));
    renderMermaid.mockResolvedValueOnce({ ok: true, svg: '<svg></svg>' });
    const { rerender, unmount } = render(<ArticleProse html={diagram} />);

    rerender(<ArticleProse html={'<pre><code class="language-mermaid">flowchart LR\nC --> D</code></pre>'} />);
    resolveFirst({ ok: true, svg: '<svg id="old"></svg>' });

    await waitFor(() => expect(screen.getByRole('img')).toBeInTheDocument());
    expect(document.querySelector('code.language-mermaid')?.textContent).toBe('flowchart LR\nC --> D');
    expect(document.body.innerHTML).not.toContain('old');
    unmount();
    expect(URL.revokeObjectURL).toHaveBeenCalledWith('blob:diagram');
  });
});
