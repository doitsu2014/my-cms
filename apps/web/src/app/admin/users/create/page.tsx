import { Home, UserPlus } from 'lucide-react';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import UserForm from '../user-form';

export default function AdminCreateUserPage() {
  return (
    <div className="space-y-6">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Users', href: '/admin/users' },
          { label: 'Create User' },
        ]}
      />

      <div>
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <UserPlus className="w-6 h-6" />
          Create User
        </h1>
        <p className="text-base-content/60 text-sm mt-1">
          Add a new administrator or writer account
        </p>
      </div>

      <UserForm />
    </div>
  );
}
