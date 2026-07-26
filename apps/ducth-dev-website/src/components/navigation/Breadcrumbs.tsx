import { Link } from 'react-router-dom';

export interface BreadcrumbItem {
  label: string;
  href?: string;
}

interface BreadcrumbsProps {
  items: BreadcrumbItem[];
}

const Breadcrumbs = ({ items }: BreadcrumbsProps) => (
  <nav className="breadcrumbs" aria-label="Breadcrumb">
    <ol>
      {items.map((item, index) => {
        const isCurrent = index === items.length - 1 || !item.href;
        return (
          <li key={`${item.label}-${index}`}>
            {isCurrent ? <span aria-current="page">{item.label}</span> : <Link to={item.href!}>{item.label}</Link>}
          </li>
        );
      })}
    </ol>
  </nav>
);

export default Breadcrumbs;
