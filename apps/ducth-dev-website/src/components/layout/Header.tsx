import { useTranslation } from 'react-i18next';
import { useEffect } from 'react';
import { SITE_CONFIG } from '../../config/site.config';

const SUPPORTED_LANGS = ['en', 'vi'];

// Get current language from URL path
const getCurrentLang = () => {
  if (typeof window === 'undefined') return 'en';
  const pathSegments = window.location.pathname.split('/').filter(Boolean);
  const langFromPath = pathSegments[0];
  return SUPPORTED_LANGS.includes(langFromPath) ? langFromPath : 'en';
};

const Header = () => {
  const { t, i18n } = useTranslation();
  const currentLang = getCurrentLang();

  // Sync i18n with URL language parameter
  useEffect(() => {
    if (i18n.language !== currentLang) {
      i18n.changeLanguage(currentLang);
    }
  }, [currentLang, i18n]);

  // Build language switch URL (hard navigation)
  const getLanguageUrl = (newLang: string) => {
    if (typeof window === 'undefined') return `/${newLang}`;
    const currentPath = window.location.pathname;
    const pathWithoutLang = currentPath.replace(`/${currentLang}`, '');
    return `/${newLang}${pathWithoutLang || ''}`;
  };

  return (
    <header className="bg-base-200 shadow-lg">
      <div className="container mx-auto flex justify-between items-center py-2 px-4">
        <a href={`/${currentLang}`} className="btn btn-ghost btn-circle avatar">
          <div className="w-10 rounded-full">
            <img src={SITE_CONFIG.avatarUrl} alt="Duc Tran's Blog" />
          </div>
        </a>
        <div className="flex items-center gap-4">
          <ul className="menu menu-horizontal px-1">
            <li>
              <a href={`/${currentLang}`}>{t('home')}</a>
            </li>
            <li>
              <a href={`/${currentLang}/categories`}>{t('categories')}</a>
            </li>
          </ul>
          <div className="dropdown dropdown-end">
            <label tabIndex={0} className="btn btn-sm btn-ghost">
              {currentLang.toUpperCase()}
            </label>
            <ul tabIndex={0} className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-32">
              <li>
                <a href={getLanguageUrl('en')} className={currentLang === 'en' ? 'active' : ''}>
                  English
                </a>
              </li>
              <li>
                <a href={getLanguageUrl('vi')} className={currentLang === 'vi' ? 'active' : ''}>
                  Tiếng Việt
                </a>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </header>
  );
};

export default Header;
