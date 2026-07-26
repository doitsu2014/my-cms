import { Link, useLocation } from 'react-router-dom';
import { useEffect, useRef, useState } from 'react';

interface MobileNavigationProps {
  lang?: string;
}

const MobileNavigation = ({ lang }: MobileNavigationProps) => {
  const location = useLocation();
  const currentLang = lang || (location.pathname.startsWith('/vi') ? 'vi' : 'en');
  const [open, setOpen] = useState(false);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const firstLinkRef = useRef<HTMLAnchorElement>(null);
  const previousPathRef = useRef(location.pathname);

  useEffect(() => {
    if (!open) return;
    firstLinkRef.current?.focus();

    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        setOpen(false);
        triggerRef.current?.focus();
      }
    };
    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [open]);

  useEffect(() => {
    if (previousPathRef.current !== location.pathname) {
      previousPathRef.current = location.pathname;
      if (open) {
        window.setTimeout(() => {
          setOpen(false);
          triggerRef.current?.focus();
        }, 0);
      }
    }
  }, [location.pathname, open]);

  return (
    <div className="mobile-navigation">
      <button
        ref={triggerRef}
        type="button"
        className="menu-trigger"
        aria-label="Menu"
        aria-expanded={open}
        aria-controls="mobile-nav"
        onClick={() => setOpen((current) => !current)}
      >
        <span aria-hidden="true">☰</span>
        <span>Menu</span>
      </button>
      {open && (
        <div className="mobile-navigation__backdrop" aria-hidden="true" onClick={() => setOpen(false)} />
      )}
      <nav
        id="mobile-nav"
        className="mobile-navigation__drawer"
        aria-label="Menu"
        aria-modal="true"
        role="dialog"
        hidden={!open}
      >
        <div className="mobile-navigation__header">
          <span className="eyebrow">Navigation</span>
          <button type="button" className="menu-close" onClick={() => {
            setOpen(false);
            triggerRef.current?.focus();
          }}>
            Close
          </button>
        </div>
        <Link ref={firstLinkRef} to={`/${currentLang}`} onClick={() => setOpen(false)}>
          {currentLang === 'vi' ? 'Trang chủ' : 'Home'}
        </Link>
        <Link to={`/${currentLang}/categories`} onClick={() => setOpen(false)}>
          {currentLang === 'vi' ? 'Danh mục' : 'Categories'}
        </Link>
        <Link to={`/${currentLang}/about`} onClick={() => setOpen(false)}>
          {currentLang === 'vi' ? 'Về tôi' : 'About'}
        </Link>
      </nav>
    </div>
  );
};

export default MobileNavigation;
