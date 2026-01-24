import { Home } from 'lucide-react';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import BlogForm from '../blog-form';

export default function AdminCreateBlogPage() {
  return (
    <div className="container-fluid mx-auto">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Blogs', href: '/admin/blogs' },
          { label: 'Create Blog' }
        ]}
      />
      <div>
        <h1 className="text-2xl font-bold mb-4">Create Blog</h1>
        <BlogForm id={undefined} />
      </div>
    </div>
  );
}
