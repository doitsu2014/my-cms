import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '@/auth/AuthContext';
import { UserRoleEnum } from '@/domains/user';

interface AdminOnlyRouteProps {
  children: ReactNode;
}

export function AdminOnlyRoute({ children }: AdminOnlyRouteProps) {
  const { user } = useAuth();
  const roles = (user?.app_metadata?.roles ?? []) as string[];
  const isAdmin = roles.includes(UserRoleEnum.Administrator);

  if (!isAdmin) {
    return (
      <div className="alert alert-error m-4">
        <span>You do not have permission to access this page.</span>
        <Link to="/admin" className="btn btn-sm">Back to admin</Link>
      </div>
    );
  }
  return <>{children}</>;
}
