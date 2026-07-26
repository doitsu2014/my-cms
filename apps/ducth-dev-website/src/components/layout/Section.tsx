import type { ElementType, ReactNode } from 'react';

export type SectionTone = 'parchment' | 'fresh-paper' | 'ink';

interface SectionProps {
  children: ReactNode;
  tone?: SectionTone;
  className?: string;
  as?: ElementType;
}

const Section = ({ children, tone = 'parchment', className = '', as: Element = 'section' }: SectionProps) => (
  <Element className={`site-section site-section--${tone} ${className}`.trim()}>{children}</Element>
);

export default Section;

