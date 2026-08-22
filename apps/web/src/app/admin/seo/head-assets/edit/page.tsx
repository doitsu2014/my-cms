import { Home } from 'lucide-react';
import { useParams } from 'react-router-dom';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import HeadAssetForm from '../head-asset-form';
export default function AdminEditSeoHeadAssetPage() { const { id } = useParams<{ id: string }>(); return <div className="space-y-6"><Breadcrumbs items={[{ label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> }, { label: 'SEO head assets', href: '/admin/seo/head-assets' }, { label: 'Edit' }]} /><div><h1 className="text-2xl font-bold">Edit SEO head asset</h1><p className="text-base-content/60 text-sm mt-1">Replace the complete source and publication state.</p></div><HeadAssetForm id={id} /></div>; }
