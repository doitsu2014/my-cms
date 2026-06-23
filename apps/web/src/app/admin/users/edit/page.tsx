import { Home, UserCog } from 'lucide-react';
import { useParams } from 'react-router-dom';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import UserForm from '../user-form';

export default function AdminEditUserPage() {
  const { id } = useParams<{ id: string }>();

  return (
    <div className="space-y-6">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Users', href: '/admin/users' },
          { label: 'Edit User' },
        ]}
      />

      <div>
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <UserCog className="w-6 h-6" />
          Edit User
        </h1>
        <p className="text-base-content/60 text-sm mt-1">
          Update email, role, or ban status for this user
        </p>
      </div>

      <UserForm id={id} />
    </div>
  );
}
