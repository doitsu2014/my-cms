import { formatPublishedDate } from '../../lib/i18n/formatPublishedDate';

interface PostMetaProps {
  category?: string;
  date: string;
  lang: string;
  readingTime?: string | number | null;
}

const PostMeta = ({ category, date, lang, readingTime }: PostMetaProps) => (
  <div className="post-meta">
    {category && <span>{category}</span>}
    {category && <span aria-hidden="true">·</span>}
    <time dateTime={date}>{formatPublishedDate(date, lang)}</time>
    {readingTime && (
      <>
        <span aria-hidden="true">·</span>
        <span>{readingTime}</span>
      </>
    )}
  </div>
);

export default PostMeta;
