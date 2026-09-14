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
if (!document.elementFromPoint) {
  document.elementFromPoint = () => null;
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

  it('keeps a preview snapshot when editor content changes after opening', async () => {
    const user = userEvent.setup();
    const { rerender, container } = render(<TipTapEditor defaultValue="<p>Original preview</p>" />);

    await user.click(await screen.findByRole('button', { name: 'Mermaid diagram' }));
    await user.click(container.querySelector('[data-tip="Preview"] button') as HTMLButtonElement);
    expect(await screen.findByRole('dialog', { name: 'Article Preview' })).toHaveTextContent('flowchart LR');

    rerender(<TipTapEditor defaultValue="<p>Changed editor value</p>" />);

    expect(screen.getByRole('dialog', { name: 'Article Preview' })).toHaveTextContent('flowchart LR');
    expect(screen.getByRole('dialog', { name: 'Article Preview' })).not.toHaveTextContent('Changed editor value');
  });

  it('does not replace an author draft when a background refresh changes default content', async () => {
    const user = userEvent.setup();
    const { container, rerender } = render(<TipTapEditor defaultValue="<p>Original draft</p>" />);
    const editable = await waitFor(() => {
      const element = container.querySelector('[contenteditable="true"]');
      expect(element).toBeTruthy();
      return element as HTMLElement;
    });

    await user.click(editable);
    await user.keyboard(' local edit');
    rerender(<TipTapEditor defaultValue="<p>Background refresh</p>" />);

    expect(editable).toHaveTextContent('local edit');
    expect(editable).not.toHaveTextContent('Background refresh');
  });
});
