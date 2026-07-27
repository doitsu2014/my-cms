import type { ReactNode } from 'react';
import { useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { getRouteLanguage } from '../../lib/i18n/getRouteLanguage';
import SiteFooter from './SiteFooter';
import SiteHeader from './SiteHeader';

interface SiteLayoutProps {
  children: ReactNode;
}

const SiteLayout = ({ children }: SiteLayoutProps) => {
  const location = useLocation();
  const lang = getRouteLanguage(location.pathname);

  useEffect(() => {
    document.documentElement.lang = lang;
  }, [lang]);

  return (
    <div className="site-layout">
      <a className="skip-link" href="#main">
        {lang === 'vi' ? 'Bỏ qua đến nội dung' : 'Skip to content'}
      </a>
      <SiteHeader />
      <main id="main" className="site-main" tabIndex={-1}>
        {children}
      </main>
      <SiteFooter />
    </div>
  );
};

export default SiteLayout;
