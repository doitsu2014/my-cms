import { useState } from 'react';
import StatusMessage from '../feedback/StatusMessage';

interface ShareActionsProps {
  canonicalUrl: string;
  title: string;
  lang: string;
}

const ShareActions = ({ canonicalUrl, title, lang }: ShareActionsProps) => {
  const [copied, setCopied] = useState(false);
  const encodedUrl = encodeURIComponent(canonicalUrl);
  const encodedTitle = encodeURIComponent(title);
  const copyLabel = lang === 'vi' ? 'Sao chép liên kết' : 'Copy link';

  const handleCopy = async () => {
    if (!navigator.clipboard?.writeText) return;
    try {
      await navigator.clipboard.writeText(canonicalUrl);
      setCopied(true);
    } catch {
      setCopied(false);
    }
  };

  return (
    <div className="share-actions">
      <span className="share-actions__label">{lang === 'vi' ? 'Chia sẻ' : 'Share'}</span>
      <a
        className="share-actions__control"
        href={`https://x.com/intent/post?url=${encodedUrl}&text=${encodedTitle}`}
        target="_blank"
        rel="noopener"
        aria-label={lang === 'vi' ? 'Chia sẻ qua X' : 'Share on X'}
      >X</a>
      <a
        className="share-actions__control"
        href={`https://www.linkedin.com/sharing/share-offsite/?url=${encodedUrl}`}
        target="_blank"
        rel="noopener noreferrer"
        aria-label={lang === 'vi' ? 'Chia sẻ qua LinkedIn' : 'Share on LinkedIn'}
      >in</a>
      <button type="button" className="share-actions__control" onClick={handleCopy} aria-label={copyLabel}>
        {lang === 'vi' ? 'Sao chép' : 'Copy'}
      </button>
      {copied && <StatusMessage>{lang === 'vi' ? 'Đã sao chép liên kết' : 'Link copied'}</StatusMessage>}
    </div>
  );
};

export default ShareActions;
