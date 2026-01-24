import { Home } from 'lucide-react';
import { useParams } from 'react-router-dom';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import BlogForm from '../blog-form';

export default function AdminEditBlogPage() {
  const { id } = useParams<{ id: string }>();

  return (
    <div className="container-fluid mx-auto">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Blogs', href: '/admin/blogs' },
          { label: 'Edit Blog' },
          { label: id || '' }
        ]}
      />
      <div>
        <h1 className="text-2xl font-bold mb-4">Edit Blog</h1>
        <BlogForm id={id} />
      </div>
    </div>
  );
}
