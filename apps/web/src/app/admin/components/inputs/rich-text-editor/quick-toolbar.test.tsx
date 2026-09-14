import '@testing-library/jest-dom/vitest';
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { QuickToolbar } from './quick-toolbar';

describe('QuickToolbar', () => {
  it('renders exactly the three requested actions and clamps the menu to the viewport', () => {
    const { container } = render(<QuickToolbar position={{ x: 9999, y: 9999 }} onAction={vi.fn()} onClose={vi.fn()} />);
    const menu = screen.getByRole('menu', { name: 'Quick Toolbar' });
    expect(screen.getAllByRole('menuitem')).toHaveLength(3);
    expect(screen.getByRole('menuitem', { name: 'Add Image' })).toBeInTheDocument();
    expect(screen.getByRole('menuitem', { name: 'Add Video' })).toBeInTheDocument();
    expect(screen.getByRole('menuitem', { name: 'Add Code Block' })).toBeInTheDocument();
    expect(menu).toHaveStyle({ left: `${window.innerWidth - 12}px`, top: `${window.innerHeight - 12}px` });
    expect(container.querySelectorAll('[role="menuitem"]')).toHaveLength(3);
  });

  it('focuses its first action and dismisses on Escape or an outside pointer without selecting an action', () => {
    const onAction = vi.fn();
    const onClose = vi.fn();
    const { rerender } = render(<QuickToolbar position={{ x: 12, y: 24 }} onAction={onAction} onClose={onClose} />);
    expect(screen.getByRole('menuitem', { name: 'Add Image' })).toHaveFocus();
    fireEvent.keyDown(document, { key: 'Escape' });
    expect(onClose).toHaveBeenCalledTimes(1);
    expect(onAction).not.toHaveBeenCalled();

    rerender(<QuickToolbar position={{ x: 12, y: 24 }} onAction={onAction} onClose={onClose} />);
    fireEvent.pointerDown(document.body);
    expect(onClose).toHaveBeenCalledTimes(2);
    expect(onAction).not.toHaveBeenCalled();
  });
});
