import { useEffect, useRef } from 'react';
import {
  defaultMermaidPresentationLabels,
  enhanceMermaidBlocks,
  type MermaidPresentationLabels,
} from './mermaid-enhancement';

export interface ArticleProseProps {
  html: string;
  mermaidLabels?: Partial<MermaidPresentationLabels>;
}

export const ArticleProse = ({ html, mermaidLabels }: ArticleProseProps) => {
  const root = useRef<HTMLDivElement>(null);
  const labels = { ...defaultMermaidPresentationLabels, ...mermaidLabels };

  useEffect(() => {
    if (!root.current || !root.current.querySelector('pre > code.language-mermaid')) return;
    return enhanceMermaidBlocks(root.current, labels);
  }, [html, labels.failure, labels.image, labels.loading, labels.source]);

  return <div ref={root} className="article-prose" dangerouslySetInnerHTML={{ __html: html }} />;
};
