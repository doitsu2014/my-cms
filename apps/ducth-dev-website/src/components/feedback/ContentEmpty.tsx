import { Link } from 'react-router-dom';

interface ContentEmptyProps {
  lang: string;
  message: string;
  href?: string;
}

const ContentEmpty = ({ lang, message, href = `/${lang}` }: ContentEmptyProps) => (
  <div className="content-feedback content-feedback--empty">
    <p>{message}</p>
    <Link className="text-link" to={href}>
      {lang === 'vi' ? 'Về trang chủ' : 'Back home'} <span aria-hidden="true">→</span>
    </Link>
  </div>
);

export default ContentEmpty;
