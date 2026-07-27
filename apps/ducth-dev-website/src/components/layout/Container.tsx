import type { ElementType, ReactNode } from 'react';

interface ContainerProps {
  children: ReactNode;
  as?: ElementType;
  className?: string;
}

const Container = ({ children, as: Element = 'div', className = '' }: ContainerProps) => (
  <Element className={`site-container ${className}`.trim()}>{children}</Element>
);

export default Container;
