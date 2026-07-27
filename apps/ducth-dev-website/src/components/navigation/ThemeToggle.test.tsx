import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ThemeToggle from './ThemeToggle';

describe('ThemeToggle', () => {
  beforeEach(() => {
    window.localStorage.clear();
    document.documentElement.dataset.theme = '';
    window.matchMedia = (query: string) =>
      ({
        matches: false,
        media: query,
        addEventListener: () => {},
        removeEventListener: () => {},
        addListener: () => {},
        removeListener: () => {},
        dispatchEvent: () => false,
        onchange: null,
      }) as unknown as MediaQueryList;
  });

  afterEach(() => {
    window.localStorage.clear();
  });

  it('renders a button with accessible label', () => {
    render(<ThemeToggle />);
    const button = screen.getByRole('button');
    expect(button.getAttribute('aria-label')).toMatch(/theme/i);
  });

  it('switches theme when clicked', async () => {
    const user = userEvent.setup();
    render(<ThemeToggle />);
    const button = screen.getByRole('button');
    const initialLabel = button.getAttribute('aria-label') ?? '';

    await user.click(button);

    expect(button.getAttribute('aria-label')).not.toBe(initialLabel);
    expect(['light', 'dark']).toContain(document.documentElement.dataset.theme);
  });

  it('persists toggle choice to localStorage', async () => {
    const user = userEvent.setup();
    render(<ThemeToggle />);
    await user.click(screen.getByRole('button'));
    expect(window.localStorage.getItem('ducth-theme')).toBeTruthy();
  });
});