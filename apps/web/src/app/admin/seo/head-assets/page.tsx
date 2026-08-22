import { Plus, Trash2, Pencil, Home } from 'lucide-react';
import { useCallback, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import { useAuth } from '@/auth/AuthContext';
import { deleteSeoHeadAsset, listSeoHeadAssets, type SeoHeadAsset } from '@/domains/seo-head-asset';

export default function AdminSeoHeadAssetsPage() {
  const { token } = useAuth(); const [assets, setAssets] = useState<SeoHeadAsset[]>([]); const [loading, setLoading] = useState(true); const [failure, setFailure] = useState(false); const [deleting, setDeleting] = useState<string | null>(null);
  const load = useCallback(async () => { setLoading(true); setFailure(false); try { setAssets(await listSeoHeadAssets(token)); } catch { setFailure(true); } finally { setLoading(false); } }, [token]);
  useEffect(() => { void load(); }, [load]);
  const remove = async (asset: SeoHeadAsset) => { if (!window.confirm(`Delete “${asset.label}”?`)) return; setDeleting(asset.id); try { await deleteSeoHeadAsset(token, asset.id); setAssets((current) => current.filter((item) => item.id !== asset.id)); } catch { setFailure(true); } finally { setDeleting(null); } };
  return <div className="space-y-6"><Breadcrumbs items={[{ label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> }, { label: 'SEO head assets' }]} /><div className="flex flex-wrap items-start justify-between gap-4"><div><h1 className="text-2xl font-bold">SEO head assets</h1><p className="text-base-content/60 text-sm mt-1">Manage validated global source for the Ducth website.</p></div><Link to="/admin/seo/head-assets/create" className="btn btn-primary"><Plus className="w-4 h-4" />Add asset</Link></div>
    {loading && <div role="status" className="loading loading-spinner" aria-label="Loading assets" />}
    {!loading && failure && <div className="alert alert-error"><span>Unable to load SEO assets.</span><button className="btn btn-sm" onClick={() => void load()}>Retry</button></div>}
    {!loading && !failure && assets.length === 0 && <div className="card bg-base-200"><div className="card-body"><h2 className="card-title">No head assets yet</h2><p>Add a reviewed verification tag, JSON-LD block, or measurement snippet.</p></div></div>}
    {!loading && !failure && assets.length > 0 && <div className="overflow-x-auto"><table className="table"><thead><tr><th>Label</th><th>Status</th><th>Order</th><th className="text-right">Actions</th></tr></thead><tbody>{assets.map((asset) => <tr key={asset.id}><td><div className="font-medium">{asset.label}</div><div className="text-xs text-base-content/60">Updated {new Date(asset.updatedAt).toLocaleString()}</div></td><td><span className={`badge ${asset.enabled ? 'badge-success' : 'badge-ghost'}`}>{asset.enabled ? 'Published' : 'Disabled'}</span></td><td>{asset.sortOrder}</td><td><div className="flex justify-end gap-2"><Link className="btn btn-sm btn-ghost" to={`/admin/seo/head-assets/edit/${asset.id}`} aria-label={`Edit ${asset.label}`}><Pencil className="w-4 h-4" /></Link><button className="btn btn-sm btn-ghost text-error" disabled={deleting === asset.id} onClick={() => void remove(asset)} aria-label={`Delete ${asset.label}`}><Trash2 className="w-4 h-4" /></button></div></td></tr>)}</tbody></table></div>}
  </div>;
}
