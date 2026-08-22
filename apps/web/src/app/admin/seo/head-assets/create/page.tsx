import { Home } from 'lucide-react';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import HeadAssetForm from '../head-asset-form';
export default function AdminCreateSeoHeadAssetPage() { return <div className="space-y-6"><Breadcrumbs items={[{ label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> }, { label: 'SEO head assets', href: '/admin/seo/head-assets' }, { label: 'Create' }]} /><div><h1 className="text-2xl font-bold">Add SEO head asset</h1><p className="text-base-content/60 text-sm mt-1">Add reviewed head-safe source for the public website.</p></div><HeadAssetForm /></div>; }
