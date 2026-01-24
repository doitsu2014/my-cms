import { Home, FolderPlus } from 'lucide-react';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import CategoryForm from '../category-form';

export default function AdminCreateCategoryPage() {
  return (
    <div className="space-y-6">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Categories', href: '/admin/categories' },
          { label: 'Create Category' },
        ]}
      />

      <div>
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <FolderPlus className="w-6 h-6" />
          Create Category
        </h1>
        <p className="text-base-content/60 text-sm mt-1">
          Add a new category to organize your content
        </p>
      </div>

      <CategoryForm id={undefined} />
    </div>
  );
}
