import { Link, useLocation } from 'react-router-dom';
import LanguageSwitch from '../navigation/LanguageSwitch';
import MobileNavigation from '../navigation/MobileNavigation';

const SiteHeader = () => {
  const location = useLocation();
  const lang = location.pathname.startsWith('/vi') ? 'vi' : 'en';
  const navigation = [
    { key: 'home', href: `/${lang}`, label: lang === 'vi' ? 'Trang chủ' : 'Home' },
    { key: 'categories', href: `/${lang}/categories`, label: lang === 'vi' ? 'Danh mục' : 'Categories' },
    { key: 'about', href: `/${lang}/about`, label: lang === 'vi' ? 'Về tôi' : 'About' },
  ];

  const isActive = (key: string, href: string) => {
    if (key === 'home') return location.pathname === href;
    return location.pathname === href || location.pathname.startsWith(`${href}/`);
  };

  return (
    <header className="site-header">
      <div className="site-container site-header__inner">
        <Link className="wordmark" to={`/${lang}`} aria-label="Đức Trần — home">
          <img src="/images/avatar.jpg" alt="" className="wordmark__avatar" />
          <span>Đức Trần<span className="wordmark__seal">.</span></span>
        </Link>
        <nav className="site-nav" aria-label={lang === 'vi' ? 'Chính' : 'Primary'}>
          {navigation.map((item) => (
            <Link
              key={item.key}
              to={item.href}
              className="site-nav__link"
              aria-current={isActive(item.key, item.href) ? 'page' : undefined}
            >
              {item.label}
            </Link>
          ))}
        </nav>
        <div className="site-header__actions">
          <LanguageSwitch />
          <MobileNavigation lang={lang} />
        </div>
      </div>
    </header>
  );
};

export default SiteHeader;
