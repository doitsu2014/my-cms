export interface ArticleProseProps {
  html: string;
}

export const ArticleProse = ({ html }: ArticleProseProps) => (
  <div className="article-prose" dangerouslySetInnerHTML={{ __html: html }} />
);
