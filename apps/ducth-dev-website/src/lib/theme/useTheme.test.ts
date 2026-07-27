import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { act, renderHook } from '@testing-library/react';
import { useTheme } from './useTheme';

const STORAGE_KEY = 'ducth-theme';

const setStoredTheme = (value: string | null) => {
  if (value === null) {
    window.localStorage.removeItem(STORAGE_KEY);
  } else {
    window.localStorage.setItem(STORAGE_KEY, value);
  }
};

describe('useTheme', () => {
  beforeEach(() => {
    document.documentElement.dataset.theme = '';
    setStoredTheme(null);
    window.matchMedia = vi.fn().mockImplementation((query: string) => ({
      matches: query.includes('dark'),
      media: query,
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
    }));
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('reads stored theme on first render', () => {
    setStoredTheme('dark');
    const { result } = renderHook(() => useTheme());
    expect(result.current.theme).toBe('dark');
  });

  it('falls back to light when no stored value', () => {
    const { result } = renderHook(() => useTheme());
    expect(result.current.theme).toBe('light');
  });

  it('applies data-theme to document when theme changes', () => {
    const { result } = renderHook(() => useTheme());
    act(() => result.current.setTheme('dark'));
    expect(document.documentElement.dataset.theme).toBe('dark');
  });

  it('persists theme to localStorage when changed', () => {
    setStoredTheme('light');
    const { result } = renderHook(() => useTheme());
    act(() => result.current.toggle());
    expect(window.localStorage.getItem(STORAGE_KEY)).toBe('dark');
  });

  it('toggle flips between light and dark', () => {
    setStoredTheme('light');
    const { result } = renderHook(() => useTheme());
    expect(result.current.theme).toBe('light');
    act(() => result.current.toggle());
    expect(result.current.theme).toBe('dark');
    act(() => result.current.toggle());
    expect(result.current.theme).toBe('light');
  });
});