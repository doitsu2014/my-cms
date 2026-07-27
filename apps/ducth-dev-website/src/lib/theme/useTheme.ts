import { useEffect, useState } from 'react';

export type ThemePreference = 'light' | 'dark';

export interface UseThemeResult {
  theme: ThemePreference;
  toggle: () => void;
  setTheme: (theme: ThemePreference) => void;
}

const STORAGE_KEY = 'ducth-theme';

const isThemePreference = (value: unknown): value is ThemePreference =>
  value === 'light' || value === 'dark';

const readStoredTheme = (): ThemePreference | null => {
  if (typeof window === 'undefined') return null;
  try {
    const stored = window.localStorage.getItem(STORAGE_KEY);
    return isThemePreference(stored) ? stored : null;
  } catch {
    return null;
  }
};

const writeStoredTheme = (theme: ThemePreference): void => {
  if (typeof window === 'undefined') return;
  try {
    window.localStorage.setItem(STORAGE_KEY, theme);
  } catch {
    /* storage unavailable; ignore */
  }
};

const applyTheme = (theme: ThemePreference): void => {
  if (typeof document === 'undefined') return;
  document.documentElement.dataset.theme = theme;
};

export const useTheme = (): UseThemeResult => {
  const [theme, setThemeState] = useState<ThemePreference>(() => {
    return readStoredTheme() ?? 'light';
  });

  useEffect(() => {
    applyTheme(theme);
    writeStoredTheme(theme);
  }, [theme]);

  const setTheme = (next: ThemePreference) => setThemeState(next);
  const toggle = () => setThemeState((prev) => (prev === 'dark' ? 'light' : 'dark'));

  return { theme, setTheme, toggle };
};