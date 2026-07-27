import { Link } from 'react-router-dom';

interface NotFoundStateProps {
  lang: string;
  message?: string;
}

const NotFoundState = ({ lang, message }: NotFoundStateProps) => (
  <div className="content-feedback content-feedback--not-found">
    <p className="eyebrow">404</p>
    <h1 className="display-h1">{lang === 'vi' ? 'Không tìm thấy' : 'Not found'}</h1>
    <p>{message || (lang === 'vi' ? 'Nội dung này không tồn tại.' : 'This content does not exist.')}</p>
    <Link className="text-link" to={`/${lang}/categories`}>
      {lang === 'vi' ? 'Xem danh mục' : 'Browse categories'} <span aria-hidden="true">→</span>
    </Link>
  </div>
);

export default NotFoundState;
