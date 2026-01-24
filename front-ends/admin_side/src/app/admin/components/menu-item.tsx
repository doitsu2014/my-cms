import { Link, useLocation } from 'react-router-dom';

export default function MenuItem({
  displayName,
  slug
}: Readonly<{
  displayName: string;
  slug: string;
}>) {
  const location = useLocation();
  const isActive = location.pathname.startsWith(slug);
  
  return (
    <Link className={isActive ? 'menu-active' : ''} to={slug}>
      {displayName}
    </Link>
  );
}
