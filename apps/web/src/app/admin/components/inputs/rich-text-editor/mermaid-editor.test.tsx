import '@testing-library/jest-dom/vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

vi.mock('@/auth/AuthContext', () => ({ useAuth: () => ({ token: null }) }));

import { TipTapEditor } from './tiptap-editor';

if (!Range.prototype.getClientRects) {
  Range.prototype.getClientRects = () => [] as unknown as DOMRectList;
}
if (!Range.prototype.getBoundingClientRect) {
  Range.prototype.getBoundingClientRect = () => new DOMRect();
}

describe('Mermaid editor authoring', () => {
  it('inserts editable Mermaid source and keeps preview presentation-only', async () => {
    const user = userEvent.setup();
    const onTextChange = vi.fn();
    const { container } = render(<TipTapEditor onTextChange={onTextChange} />);

    await user.click(await screen.findByRole('button', { name: 'Mermaid diagram' }));
    const code = container.querySelector('pre code.language-mermaid');
    expect(code).toHaveTextContent('flowchart LR');

    const savedSource = code?.textContent;
    await user.click(container.querySelector('[data-tip="Preview"] button') as HTMLButtonElement);
    expect(screen.getByRole('dialog', { name: 'Article Preview' })).toBeInTheDocument();
    await waitFor(() => expect(screen.getByRole('button', { name: 'Close article preview' })).toHaveFocus());
    expect(container.querySelector('pre code.language-mermaid')?.textContent).toBe(savedSource);
  });

  it('keeps Mermaid authoring unavailable in read-only mode', async () => {
    render(<TipTapEditor readOnly defaultValue="<p>Read only</p>" />);
    expect(await screen.findByRole('button', { name: 'Mermaid diagram' })).toBeDisabled();
  });
});
