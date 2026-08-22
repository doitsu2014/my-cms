import { zodResolver } from '@hookform/resolvers/zod';
import { useEffect, useRef, useState } from 'react';
import { useForm } from 'react-hook-form';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '@/auth/AuthContext';
import { createSeoHeadAsset, getSeoHeadAsset, SeoHeadAssetApiError, seoHeadAssetFormSchema, updateSeoHeadAsset, type SeoHeadAssetFormValues } from '@/domains/seo-head-asset';

export default function HeadAssetForm({ id }: { id?: string }) {
  const { token } = useAuth();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(Boolean(id));
  const [failure, setFailure] = useState<string | null>(null);
  const [saveFailure, setSaveFailure] = useState<string | null>(null);
  const rowVersion = useRef<number | undefined>();
  const { register, reset, handleSubmit, formState: { errors, isSubmitting } } = useForm<SeoHeadAssetFormValues>({ resolver: zodResolver(seoHeadAssetFormSchema), defaultValues: { label: '', html: '', enabled: true, sortOrder: 1 } });

  const load = async () => {
    if (!id) return;
    setLoading(true); setFailure(null);
    try { const asset = await getSeoHeadAsset(token, id); rowVersion.current = asset.rowVersion; reset({ label: asset.label, html: asset.html, enabled: asset.enabled, sortOrder: asset.sortOrder }); }
    catch { setFailure('Unable to load this asset.'); } finally { setLoading(false); }
  };
  useEffect(() => { void load(); }, [id, token]);

  const onSubmit = async (values: SeoHeadAssetFormValues) => {
    setSaveFailure(null);
    try { if (id && rowVersion.current !== undefined) await updateSeoHeadAsset(token, id, values, rowVersion.current); else await createSeoHeadAsset(token, values); navigate('/admin/seo/head-assets'); }
    catch (error) { setSaveFailure(error instanceof SeoHeadAssetApiError && error.status === 409 ? 'This asset changed elsewhere. Reload it before saving.' : 'Unable to save this asset.'); }
  };

  if (loading) return <div role="status" className="loading loading-spinner" aria-label="Loading asset" />;
  if (failure) return <div className="alert alert-error"><span>{failure}</span><button className="btn btn-sm" onClick={() => void load()}>Retry</button></div>;
  return <form className="space-y-5 max-w-3xl" onSubmit={handleSubmit(onSubmit)} noValidate>
    {saveFailure && <div role="alert" className="alert alert-error">{saveFailure}</div>}
    <div className="form-control"><label className="label" htmlFor="seo-label"><span className="label-text">Label</span></label><input id="seo-label" className="input input-bordered" {...register('label')} aria-invalid={Boolean(errors.label)} />{errors.label && <p className="text-error text-sm mt-1" role="alert">{errors.label.message}</p>}</div>
    <div className="form-control"><label className="label" htmlFor="seo-sort-order"><span className="label-text">Sort order</span></label><input id="seo-sort-order" type="number" min="1" className="input input-bordered" {...register('sortOrder', { valueAsNumber: true })} aria-invalid={Boolean(errors.sortOrder)} />{errors.sortOrder && <p className="text-error text-sm mt-1" role="alert">{errors.sortOrder.message}</p>}</div>
    <label className="label cursor-pointer justify-start gap-3"><input type="checkbox" className="toggle" {...register('enabled')} /><span className="label-text">Publish this asset</span></label>
    <div className="form-control"><label className="label" htmlFor="seo-html"><span className="label-text">Head source</span></label><textarea id="seo-html" className="textarea textarea-bordered min-h-64 font-mono text-sm" {...register('html')} aria-describedby="seo-html-help" aria-invalid={Boolean(errors.html)} /><p id="seo-html-help" className="text-base-content/60 text-sm mt-1">Stored as inert text in this screen. It is validated before publication.</p>{errors.html && <p className="text-error text-sm mt-1" role="alert">{errors.html.message}</p>}</div>
    <div className="flex flex-wrap gap-3"><button type="submit" className="btn btn-primary" disabled={isSubmitting}>{isSubmitting ? 'Saving…' : 'Save asset'}</button><button type="button" className="btn btn-ghost" onClick={() => navigate('/admin/seo/head-assets')}>Cancel</button></div>
  </form>;
}
