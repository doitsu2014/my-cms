import StatusMessage from './StatusMessage';

const messages = {
  en: { message: 'We could not load this page.', retry: 'Retry' },
  vi: { message: 'Không thể tải trang này.', retry: 'Thử lại' },
};

interface ContentErrorProps {
  lang: string;
  onRetry: () => void;
}

export const ContentError = ({ lang, onRetry }: ContentErrorProps) => {
  const copy = messages[lang as keyof typeof messages] || messages.en;
  return (
    <div className="content-feedback content-feedback--error">
      <StatusMessage>{copy.message}</StatusMessage>
      <button type="button" className="button button--secondary" onClick={onRetry}>
        {copy.retry}
      </button>
    </div>
  );
};

export default ContentError;
