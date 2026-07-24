import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import en from './locales/en.json';
import vi from './locales/vi.json';

// Get language from URL path (/en/... or /vi/...)
const getLanguageFromPath = () => {
  // Check if we're in a browser environment
  if (typeof window === 'undefined') {
    return 'en'; // Default for server-side
  }
  const pathParts = window.location.pathname.split('/').filter(Boolean);
  const lang = pathParts[0];
  return ['en', 'vi'].includes(lang) ? lang : 'en';
};

i18n.use(initReactI18next).init({
  resources: {
    en: {
      translation: en,
    },
    vi: {
      translation: vi,
    },
  },
  lng: getLanguageFromPath(),
  fallbackLng: 'en',
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
