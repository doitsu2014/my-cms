import { useEffect, useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { toast } from 'sonner';
import {
  Database,
  Home,
  Pencil,
  Plus,
  Trash2,
  X,
  AlertTriangle,
} from 'lucide-react';
import { Link } from 'react-router-dom';
import Breadcrumbs from '../../components/my-breadcrumbs';
import TableSkeleton from '../../components/skeleton/table-skeleton';
import type { BucketModel } from '@/models/MediaModels';
import {
  createBucketFromTextFormSchema,
  type CreateBucketTextFormData,
  parseFileSizeLimit,
  parseMimeTypesList,
  updateBucketFromTextFormSchema,
  type UpdateBucketTextFormData,
} from '@/schemas/bucket.schema';
import {
  getApiUrl,
  authenticatedFetch,
  getBucketApiUrl,
  getBucketsApiUrl,
  getEmptyBucketApiUrl,
} from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';

function extractErrorMessage(payload: unknown, fallback: string): string {
  if (payload && typeof payload === 'object' && 'errors' in payload) {
    const errs = (payload as { errors?: unknown }).errors;
    if (Array.isArray(errs) && errs.length > 0 && typeof errs[0] === 'string') {
      return errs[0];
    }
  }
  return fallback;
}

function formatCreatedAt(iso: string): string {
  if (!iso) return '—';
  try {
    return new Date(iso).toLocaleDateString();
  } catch {
    return iso;
  }
}

function formatAllowedMimeTypes(types: string[] | null | undefined): string {
  if (!types || types.length === 0) return '—';
  const head = types.slice(0, 3).join(', ');
  return types.length > 3 ? `${head} +${types.length - 3} more` : head;
}

export default function AdminBucketsPage() {
  const { token } = useAuth();
  const [buckets, setBuckets] = useState<BucketModel[]>([]);
  const [pageLoading, setPageLoading] = useState(true);

  const [isCreateOpen, setIsCreateOpen] = useState(false);
  const [isEditOpen, setIsEditOpen] = useState(false);
  const [editingBucket, setEditingBucket] = useState<BucketModel | null>(null);
  const [busyCreate, setBusyCreate] = useState(false);
  const [busyUpdate, setBusyUpdate] = useState(false);

  const [emptyTarget, setEmptyTarget] = useState<BucketModel | null>(null);
  const [isEmptying, setIsEmptying] = useState(false);

  const [deleteTarget, setDeleteTarget] = useState<BucketModel | null>(null);
  const [forcePurge, setForcePurge] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const createForm = useForm<CreateBucketTextFormData>({
    resolver: zodResolver(createBucketFromTextFormSchema),
    defaultValues: { name: '', public: false, fileSizeLimit: '', allowedMimeTypes: '' },
  });

  const editForm = useForm<UpdateBucketTextFormData>({
    resolver: zodResolver(updateBucketFromTextFormSchema),
    defaultValues: { public: false, fileSizeLimit: '', allowedMimeTypes: '' },
  });

  const loadBuckets = async () => {
    try {
      setPageLoading(true);
      const response = await authenticatedFetch(
        getBucketsApiUrl(),
        token,
        { cache: 'no-store' },
      );
      const res = await response.json();
      setBuckets(Array.isArray(res.data) ? res.data : []);
    } catch (error) {
      console.error('Failed to load buckets:', error);
      toast.error('Failed to load buckets');
      setBuckets([]);
    } finally {
      setPageLoading(false);
    }
  };

  useEffect(() => {
    loadBuckets();
  }, [token]);

  const openCreate = () => {
    createForm.reset({ name: '', public: false, fileSizeLimit: '', allowedMimeTypes: '' });
    setIsCreateOpen(true);
  };

  const closeCreate = () => {
    setIsCreateOpen(false);
  };

  const submitCreate = createForm.handleSubmit(async (values) => {
    try {
      setBusyCreate(true);
      const body: Record<string, unknown> = {
        name: values.name,
        public: values.public,
      };
      const fileSizeLimit = parseFileSizeLimit(values.fileSizeLimit);
      const allowedMimeTypes = parseMimeTypesList(values.allowedMimeTypes);
      if (fileSizeLimit !== undefined) body.fileSizeLimit = fileSizeLimit;
      if (allowedMimeTypes !== undefined) body.allowedMimeTypes = allowedMimeTypes;
      const response = await authenticatedFetch(getBucketsApiUrl(), token, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
      if (!response.ok) {
        const errPayload = await response.json().catch(() => null);
        throw new Error(extractErrorMessage(errPayload, 'Failed to create bucket'));
      }
      toast.success('Bucket created successfully');
      setIsCreateOpen(false);
      await loadBuckets();
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to create bucket';
      toast.error(message);
    } finally {
      setBusyCreate(false);
    }
  });

  const openEdit = (bucket: BucketModel) => {
    setEditingBucket(bucket);
    editForm.reset({
      public: bucket.public,
      fileSizeLimit:
        bucket.fileSizeLimit === null || bucket.fileSizeLimit === undefined
          ? ''
          : String(bucket.fileSizeLimit),
      allowedMimeTypes: bucket.allowedMimeTypes ? bucket.allowedMimeTypes.join(', ') : '',
    });
    setIsEditOpen(true);
  };

  const closeEdit = () => {
    setIsEditOpen(false);
    setEditingBucket(null);
  };

  const submitEdit = editForm.handleSubmit(async (values) => {
    if (!editingBucket) return;
    try {
      setBusyUpdate(true);
      const body: Record<string, unknown> = { public: values.public };
      if (values.fileSizeLimit !== undefined) {
        if (values.fileSizeLimit === '') {
          body.fileSizeLimit = null;
        } else {
          body.fileSizeLimit = parseFileSizeLimit(values.fileSizeLimit);
        }
      }
      if (values.allowedMimeTypes !== undefined) {
        if (values.allowedMimeTypes === '') {
          body.allowedMimeTypes = null;
        } else {
          body.allowedMimeTypes = parseMimeTypesList(values.allowedMimeTypes);
        }
      }
      const response = await authenticatedFetch(
        getBucketApiUrl(editingBucket.name),
        token,
        {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(body),
        },
      );
      if (!response.ok) {
        const errPayload = await response.json().catch(() => null);
        throw new Error(extractErrorMessage(errPayload, 'Failed to update bucket'));
      }
      toast.success('Bucket updated successfully');
      setIsEditOpen(false);
      setEditingBucket(null);
      await loadBuckets();
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to update bucket';
      toast.error(message);
    } finally {
      setBusyUpdate(false);
    }
  });

  const confirmEmpty = async () => {
    if (!emptyTarget) return;
    try {
      setIsEmptying(true);
      const response = await authenticatedFetch(
        getEmptyBucketApiUrl(emptyTarget.name),
        token,
        { method: 'POST' },
      );
      if (!response.ok) {
        const errPayload = await response.json().catch(() => null);
        throw new Error(extractErrorMessage(errPayload, 'Failed to empty bucket'));
      }
      toast.success(`Bucket '${emptyTarget.name}' emptied`);
      setEmptyTarget(null);
      await loadBuckets();
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to empty bucket';
      toast.error(message);
    } finally {
      setIsEmptying(false);
    }
  };

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    try {
      setIsDeleting(true);
      const url = forcePurge
        ? `${getBucketApiUrl(deleteTarget.name)}?purge=true`
        : `${getBucketApiUrl(deleteTarget.name)}?purge=false`;
      const response = await authenticatedFetch(url, token, { method: 'DELETE' });
      if (!response.ok) {
        const errPayload = await response.json().catch(() => null);
        throw new Error(extractErrorMessage(errPayload, 'Failed to delete bucket'));
      }
      toast.success(`Bucket '${deleteTarget.name}' deleted`);
      setDeleteTarget(null);
      setForcePurge(false);
      await loadBuckets();
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to delete bucket';
      toast.error(message);
    } finally {
      setIsDeleting(false);
    }
  };

  const createErrors = createForm.formState.errors;
  const editErrors = editForm.formState.errors;

  return (
    <div className="space-y-6">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Media', href: '/admin/media' },
          { label: 'Buckets' },
        ]}
      />

      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold">Storage Buckets</h1>
          <p className="text-base-content/60 text-sm">
            Manage Supabase Storage buckets
          </p>
        </div>
        <button className="btn btn-primary gap-2" onClick={openCreate}>
          <Plus className="w-4 h-4" />
          New Bucket
        </button>
      </div>

      <div className="card bg-base-100 shadow-sm">
        <div className="overflow-x-auto">
          {pageLoading ? (
            <div className="p-4">
              <TableSkeleton rows={5} columns={6} showHeader={true} className="w-full" />
            </div>
          ) : buckets.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-16 px-4">
              <div className="bg-base-200 rounded-full p-4 mb-4">
                <Database className="w-8 h-8 text-base-content/40" />
              </div>
              <h3 className="text-lg font-semibold mb-1">No buckets yet</h3>
              <p className="text-base-content/60 text-center mb-4">
                Get started by creating your first bucket
              </p>
              <button className="btn btn-primary btn-sm gap-2" onClick={openCreate}>
                <Plus className="w-4 h-4" />
                Create Bucket
              </button>
            </div>
          ) : (
            <table className="table">
              <thead>
                <tr className="bg-base-200/50">
                  <th>Name</th>
                  <th>Public</th>
                  <th>File Size Limit</th>
                  <th>Allowed MIME Types</th>
                  <th>Created</th>
                  <th className="text-center w-44">Actions</th>
                </tr>
              </thead>
              <tbody>
                {buckets.map((bucket) => (
                  <tr key={bucket.id} className="hover:bg-base-200/30 transition-colors">
                    <td className="font-medium">
                      <code className="text-xs bg-base-200 px-2 py-1 rounded">
                        {bucket.name}
                      </code>
                    </td>
                    <td>
                      <span
                        className={`badge badge-sm ${
                          bucket.public ? 'badge-success' : 'badge-ghost'
                        }`}
                      >
                        {bucket.public ? 'Public' : 'Private'}
                      </span>
                    </td>
                    <td className="text-base-content/80 text-sm">
                      {bucket.fileSizeLimit ?? '—'}
                    </td>
                    <td className="text-base-content/80 text-sm">
                      {formatAllowedMimeTypes(bucket.allowedMimeTypes)}
                    </td>
                    <td className="text-base-content/70 text-sm">
                      {formatCreatedAt(bucket.createdAt)}
                    </td>
                    <td>
                      <div className="flex justify-center gap-1">
                        <button
                          className="btn btn-ghost btn-sm btn-square"
                          onClick={() => openEdit(bucket)}
                          title="Edit bucket"
                        >
                          <Pencil className="w-4 h-4" />
                        </button>
                        <button
                          className="btn btn-ghost btn-sm btn-square"
                          onClick={() => setEmptyTarget(bucket)}
                          title="Empty bucket"
                        >
                          <AlertTriangle className="w-4 h-4" />
                        </button>
                        <button
                          className="btn btn-ghost btn-sm btn-square text-error hover:bg-error/10 disabled:opacity-40 disabled:cursor-not-allowed"
                          onClick={() => {
                            setDeleteTarget(bucket);
                            setForcePurge(false);
                          }}
                          disabled={bucket.name === 'media'}
                          title={
                            bucket.name === 'media'
                              ? "Reserved bucket 'media' cannot be deleted"
                              : 'Delete bucket'
                          }
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </div>

      {/* Create Modal */}
      <dialog className={`modal ${isCreateOpen ? 'modal-open' : ''}`}>
        <div className="modal-box max-w-lg">
          <h3 className="font-bold text-lg mb-2">New Bucket</h3>
          <form onSubmit={submitCreate} className="space-y-4">
            <label className="form-control w-full pt-2">
              <div className="label py-1">
                <span className="label-text font-medium">Name</span>
              </div>
              <input
                type="text"
                className={`input input-bordered w-full ${
                  createErrors.name ? 'input-error' : ''
                }`}
                placeholder="my-private-bucket"
                {...createForm.register('name')}
              />
              {createErrors.name && (
                <div className="label py-1">
                  <span className="label-text-alt text-error">
                    {createErrors.name.message}
                  </span>
                </div>
              )}
            </label>

            <label className="form-control w-full">
              <span className="label-text font-medium mb-2 block">Public bucket</span>
              <div className="flex items-center gap-3">
                <input
                  type="checkbox"
                  className="toggle toggle-primary"
                  {...createForm.register('public')}
                />
                <span className="text-sm text-base-content/70">
                  When enabled, files in this bucket are publicly readable
                </span>
              </div>
            </label>

            <label className="form-control w-full">
              <div className="label py-1">
                <span className="label-text font-medium">File size limit (bytes, optional)</span>
              </div>
              <input
                type="number"
                min="1"
                className="input input-bordered w-full"
                placeholder="e.g. 5242880"
                {...createForm.register('fileSizeLimit')}
              />
              {createErrors.fileSizeLimit && (
                <div className="label py-1">
                  <span className="label-text-alt text-error">
                    {createErrors.fileSizeLimit.message}
                  </span>
                </div>
              )}
            </label>

            <label className="form-control w-full">
              <div className="label py-1">
                <span className="label-text font-medium">
                  Allowed MIME types (comma-separated, optional)
                </span>
              </div>
              <input
                type="text"
                className="input input-bordered w-full"
                placeholder="image/png, image/jpeg"
                {...createForm.register('allowedMimeTypes')}
              />
              {createErrors.allowedMimeTypes && (
                <div className="label py-1">
                  <span className="label-text-alt text-error">
                    {createErrors.allowedMimeTypes.message}
                  </span>
                </div>
              )}
            </label>

            <div className="modal-action">
              <button
                type="button"
                className="btn btn-ghost"
                onClick={closeCreate}
                disabled={busyCreate}
              >
                Cancel
              </button>
              <button type="submit" className="btn btn-primary" disabled={busyCreate}>
                {busyCreate ? (
                  <>
                    <span className="loading loading-spinner loading-sm"></span>
                    Creating...
                  </>
                ) : (
                  'Create'
                )}
              </button>
            </div>
          </form>
        </div>
        <form method="dialog" className="modal-backdrop" onClick={closeCreate}>
          <button>close</button>
        </form>
      </dialog>

      {/* Edit Modal */}
      <dialog className={`modal ${isEditOpen ? 'modal-open' : ''}`}>
        <div className="modal-box max-w-lg">
          <h3 className="font-bold text-lg mb-2">
            Edit bucket{' '}
            <code className="text-xs bg-base-200 px-2 py-1 rounded">
              {editingBucket?.name}
            </code>
          </h3>
          <form onSubmit={submitEdit} className="space-y-4">
            <label className="form-control w-full pt-2">
              <span className="label-text font-medium mb-2 block">Public bucket</span>
              <div className="flex items-center gap-3">
                <input
                  type="checkbox"
                  className="toggle toggle-primary"
                  {...editForm.register('public')}
                />
                <span className="text-sm text-base-content/70">
                  When enabled, files in this bucket are publicly readable
                </span>
              </div>
            </label>

            <label className="form-control w-full">
              <div className="label py-1">
                <span className="label-text font-medium">File size limit (bytes, blank to clear)</span>
              </div>
              <input
                type="number"
                min="1"
                className="input input-bordered w-full"
                placeholder="e.g. 10485760"
                {...editForm.register('fileSizeLimit')}
              />
              {editErrors.fileSizeLimit && (
                <div className="label py-1">
                  <span className="label-text-alt text-error">
                    {editErrors.fileSizeLimit.message}
                  </span>
                </div>
              )}
            </label>

            <label className="form-control w-full">
              <div className="label py-1">
                <span className="label-text font-medium">
                  Allowed MIME types (comma-separated, blank to clear)
                </span>
              </div>
              <input
                type="text"
                className="input input-bordered w-full"
                placeholder="image/png, image/jpeg"
                {...editForm.register('allowedMimeTypes')}
              />
              {editErrors.allowedMimeTypes && (
                <div className="label py-1">
                  <span className="label-text-alt text-error">
                    {editErrors.allowedMimeTypes.message}
                  </span>
                </div>
              )}
            </label>

            <div className="modal-action">
              <button
                type="button"
                className="btn btn-ghost"
                onClick={closeEdit}
                disabled={busyUpdate}
              >
                Cancel
              </button>
              <button type="submit" className="btn btn-primary" disabled={busyUpdate}>
                {busyUpdate ? (
                  <>
                    <span className="loading loading-spinner loading-sm"></span>
                    Saving...
                  </>
                ) : (
                  'Save'
                )}
              </button>
            </div>
          </form>
        </div>
        <form method="dialog" className="modal-backdrop" onClick={closeEdit}>
          <button>close</button>
        </form>
      </dialog>

      {/* Empty Confirmation Modal */}
      <dialog className={`modal ${emptyTarget ? 'modal-open' : ''}`}>
        <div className="modal-box">
          <h3 className="font-bold text-lg">Empty Bucket</h3>
          <p className="py-4">
            This will permanently delete all objects in{' '}
            <span className="font-semibold">'{emptyTarget?.name}'</span>. Continue?
          </p>
          <div className="modal-action">
            <button
              className="btn btn-ghost"
              onClick={() => setEmptyTarget(null)}
              disabled={isEmptying}
            >
              Cancel
            </button>
            <button
              className="btn btn-warning"
              onClick={confirmEmpty}
              disabled={isEmptying}
            >
              {isEmptying ? (
                <>
                  <span className="loading loading-spinner loading-sm"></span>
                  Emptying...
                </>
              ) : (
                'Empty'
              )}
            </button>
          </div>
        </div>
        <form method="dialog" className="modal-backdrop" onClick={() => setEmptyTarget(null)}>
          <button>close</button>
        </form>
      </dialog>

      {/* Delete Confirmation Modal */}
      <dialog className={`modal ${deleteTarget ? 'modal-open' : ''}`}>
        <div className="modal-box">
          <h3 className="font-bold text-lg">Delete Bucket</h3>
          <p className="py-2">
            Are you sure you want to delete the bucket{' '}
            <span className="font-semibold">'{deleteTarget?.name}'</span>?
          </p>
          <label className="label cursor-pointer justify-start gap-3">
            <input
              type="checkbox"
              className="checkbox checkbox-error"
              checked={forcePurge}
              onChange={(e) => setForcePurge(e.target.checked)}
            />
            <span className="label-text">Force delete with all objects</span>
          </label>
          <div className="modal-action">
            <button
              className="btn btn-ghost"
              onClick={() => {
                setDeleteTarget(null);
                setForcePurge(false);
              }}
              disabled={isDeleting}
            >
              Cancel
            </button>
            <button
              className="btn btn-error"
              onClick={confirmDelete}
              disabled={isDeleting}
            >
              {isDeleting ? (
                <>
                  <span className="loading loading-spinner loading-sm"></span>
                  Deleting...
                </>
              ) : (
                'Delete'
              )}
            </button>
          </div>
        </div>
        <form
          method="dialog"
          className="modal-backdrop"
          onClick={() => {
            setDeleteTarget(null);
            setForcePurge(false);
          }}
        >
          <button>close</button>
        </form>
      </dialog>

      <Link to={getApiUrl('/admin/media')} className="hidden">
        <X className="w-4 h-4" />
      </Link>
    </div>
  );
}
