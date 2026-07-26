interface ArticleProseProps {
  html: string;
}

const ArticleProse = ({ html }: ArticleProseProps) => (
  <div className="article-prose" dangerouslySetInnerHTML={{ __html: html }} />
);

export default ArticleProse;
