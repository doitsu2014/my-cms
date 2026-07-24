import { useTranslation } from 'react-i18next';
import { SITE_CONFIG } from '../../config/site.config';

const Footer = () => {
  const { t } = useTranslation();
  const currentYear = new Date().getFullYear();
  const { socialLinks } = SITE_CONFIG;

  return (
    <footer className="footer footer-center p-10 bg-base-200 text-base-content">
      <aside>
        <p className="font-bold">
          {t('footer')}
        </p>
        <p>{t('copyright').replace('2026', String(currentYear))}</p>
      </aside>
      <nav>
        <div className="grid grid-flow-col gap-4">
          <a
            href={socialLinks.github}
            target="_blank"
            rel="noopener noreferrer"
            className="link link-hover"
          >
            GitHub
          </a>
          <a
            href={socialLinks.twitter}
            target="_blank"
            rel="noopener noreferrer"
            className="link link-hover"
          >
            Twitter
          </a>
          <a
            href={socialLinks.linkedin}
            target="_blank"
            rel="noopener noreferrer"
            className="link link-hover"
          >
            LinkedIn
          </a>
        </div>
      </nav>
    </footer>
  );
};

export default Footer;
