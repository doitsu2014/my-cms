import type { ReactNode } from 'react';
import Container from '../layout/Container';
import Eyebrow from './Eyebrow';

interface SectionHeaderProps {
  eyebrow?: ReactNode;
  title: ReactNode;
  description?: ReactNode;
  className?: string;
}

const SectionHeader = ({ eyebrow, title, description, className = '' }: SectionHeaderProps) => (
  <Container className={`section-header ${className}`.trim()}>
    <div>
      {eyebrow && <Eyebrow>{eyebrow}</Eyebrow>}
      <h2 className="display-h2">{title}</h2>
    </div>
    {description && <p className="section-header__description">{description}</p>}
  </Container>
);

export default SectionHeader;
