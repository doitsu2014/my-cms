import { Link, useLocation } from 'react-router-dom';

import { getLocalizedPath, type SupportedLanguage } from '../../lib/i18n/getLocalizedPath';

const supportedLanguages: SupportedLanguage[] = ['en', 'vi'];
interface LanguageSwitchProps {
  className?: string;
}

export const LanguageSwitch = ({ className = '' }: LanguageSwitchProps) => {
  const location = useLocation();
  const activeLang: SupportedLanguage = location.pathname.startsWith('/vi') ? 'vi' : 'en';

  return (
    <div className={`language-switch ${className}`.trim()} role="group" aria-label="Language">
      {supportedLanguages.map((language) => (
        <Link
          key={language}
          to={getLocalizedPath(location.pathname, language, location.search, location.hash)}
          aria-current={activeLang === language ? 'true' : undefined}
          className={activeLang === language ? 'is-active' : undefined}
        >
          {language.toUpperCase()}
        </Link>
      ))}
    </div>
  );
};

export default LanguageSwitch;
