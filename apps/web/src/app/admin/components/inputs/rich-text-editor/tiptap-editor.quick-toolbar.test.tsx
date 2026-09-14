import '@testing-library/jest-dom/vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

vi.mock('@/auth/AuthContext', () => ({ useAuth: () => ({ token: 'admin-token' }) }));
vi.mock('@/config/api.config', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@/config/api.config')>()),
  authenticatedFetch: vi.fn().mockResolvedValue(new Response(JSON.stringify({ data: [] }))),
}));

import { TipTapEditor } from './tiptap-editor';

if (!Range.prototype.getClientRects) Range.prototype.getClientRects = () => [] as unknown as DOMRectList;
if (!Range.prototype.getBoundingClientRect) Range.prototype.getBoundingClientRect = () => new DOMRect();
if (!document.elementFromPoint) document.elementFromPoint = () => document.querySelector('[contenteditable="true"]');

describe('TipTapEditor Quick Toolbar', () => {
  it('keeps the native context menu path for read-only content', async () => {
    const { container } = render(<TipTapEditor readOnly defaultValue="<p>Read only</p>" />);
    const editable = await waitFor(() => container.querySelector('[contenteditable="false"]') as HTMLElement);
    const event = new MouseEvent('contextmenu', { bubbles: true, cancelable: true, clientX: 10, clientY: 10 });
    editable.dispatchEvent(event);
    expect(event.defaultPrevented).toBe(false);
    expect(screen.queryByRole('menu', { name: 'Quick Toolbar' })).not.toBeInTheDocument();
  });

  it('exposes the three-action toolbar for an editable resolved right-click and inserts a code block', async () => {
    const user = userEvent.setup();
    const { container } = render(<TipTapEditor defaultValue="<p>One</p>" />);
    const editable = await waitFor(() => container.querySelector('[contenteditable="true"]') as HTMLElement);
    fireEvent.contextMenu(editable, { clientX: 10, clientY: 10 });

    const codeAction = await screen.findByRole('menuitem', { name: 'Add Code Block' });
    await user.click(codeAction);
    expect(container.querySelector('pre')).toBeInTheDocument();
  });

  it('opens the shared image dialog from the persistent Image toolbar control', async () => {
    const user = userEvent.setup();
    const { container } = render(<TipTapEditor />);
    await waitFor(() => expect(container.querySelector('[contenteditable="true"]')).toBeTruthy());
    await user.click(container.querySelector('[data-tip="Image"] [role="button"]') as HTMLElement);
    await user.click(screen.getByRole('button', { name: 'Choose or upload images' }));
    expect(await screen.findByRole('dialog', { name: 'Insert images' })).toBeInTheDocument();
    expect(screen.getByLabelText('Upload local images')).toHaveAttribute('multiple');
  });
});
