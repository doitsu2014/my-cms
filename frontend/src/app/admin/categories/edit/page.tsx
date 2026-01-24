import { Home, FolderEdit } from 'lucide-react';
import { useParams } from 'react-router-dom';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import CategoryForm from '../category-form';

export default function AdminEditCategoryPage() {
  const { id } = useParams<{ id: string }>();

  return (
    <div className="space-y-6">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Categories', href: '/admin/categories' },
          { label: 'Edit Category' },
        ]}
      />

      <div>
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <FolderEdit className="w-6 h-6" />
          Edit Category
        </h1>
        <p className="text-base-content/60 text-sm mt-1">
          Update category details and translations
        </p>
      </div>

      <CategoryForm id={id} />
    </div>
  );
}
