import { Link, useLocation } from 'react-router-dom';
import { getRouteLanguage } from '../../lib/i18n/getRouteLanguage';
import Container from './Container';

const SiteFooter = () => {
  const location = useLocation();
  const lang = getRouteLanguage(location.pathname);
  const isProduction = typeof process !== 'undefined' && process.env.NODE_ENV === 'production';
  const copy = lang === 'vi'
    ? {
        summary: 'Ghi chép yên tĩnh về phần mềm, hệ thống và công việc phía sau chúng.',
        navigate: 'Dẫn đường',
        elsewhere: 'Nơi khác',
        contact: 'Liên kết liên hệ đã xác thực sẽ xuất hiện khi hồ sơ tác giả được công bố.',
        home: 'Trang chủ',
        categories: 'Danh mục',
        about: 'Về tôi',
      }
    : {
        summary: 'A quiet notebook for software, systems, and the work around them.',
        navigate: 'Navigate',
        elsewhere: 'Elsewhere',
        contact: 'Verified contact links will appear here when the author profile is published.',
        home: 'Home',
        categories: 'Categories',
        about: 'About',
      };

  return (
    <footer className="site-footer">
      <Container>
        <div className="site-footer__grid">
          <section>
            <h2 className="footer-heading" lang="en">Duc Tran.</h2>
            <p className="footer-copy">{copy.summary}</p>
          </section>
          <nav aria-label="Footer navigation">
            <h2 className="footer-heading">{copy.navigate}</h2>
            <ul className="footer-links">
              <li><Link to={`/${lang}`}>{copy.home}</Link></li>
              <li><Link to={`/${lang}/categories`}>{copy.categories}</Link></li>
              <li><Link to={`/${lang}/about`}>{copy.about}</Link></li>
            </ul>
          </nav>
          <section>
            <h2 className="footer-heading">{copy.elsewhere}</h2>
            <p className="footer-copy">{copy.contact}</p>
          </section>
        </div>
        <div className="site-footer__bottom">
          <span>© {new Date().getFullYear()} Duc Tran.</span>
          {!isProduction && <span data-testid="build-note">Built for the reader · development build</span>}
        </div>
      </Container>
    </footer>
  );
};

export default SiteFooter;
