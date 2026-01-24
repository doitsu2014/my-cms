import { Link } from 'react-router-dom';

type BreadcrumbItem = {
  label: string;
  href?: string;
  icon?: React.ReactNode;
};

type BreadcrumbsProps = {
  items: BreadcrumbItem[];
};

export default function Breadcrumbs({ items }: BreadcrumbsProps) {
  return (
    <div className="breadcrumbs text-sm mb-4">
      <ul>
        {items.map((item, index) => (
          <li key={index}>
            {item.href ? (
              <Link to={item.href} className="inline-flex items-center gap-2 hover:text-primary transition-colors">
                {item.icon}
                {item.label}
              </Link>
            ) : (
              <span className="inline-flex items-center gap-2 opacity-70">
                {item.icon}
                {item.label}
              </span>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
