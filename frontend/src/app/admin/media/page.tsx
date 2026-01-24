import { useEffect, useState, useMemo } from 'react';
import { toast } from 'sonner';
import {
  Home,
  Plus,
  Search,
  X,
  FolderOpen,
  Trash2,
  CheckSquare,
  Square,
  RefreshCw,
} from 'lucide-react';
import Breadcrumbs from '../components/my-breadcrumbs';
import MediaGridItem from './components/media-grid-item';
import MediaUploadModal from './components/media-upload-modal';
import MediaPreviewModal from './components/media-preview-modal';
import type { MediaMetadata } from '@/models/MediaModels';
import { getFileName } from '@/models/MediaModels';
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';

export default function AdminMediaPage() {
  const { token, keycloak } = useAuth();
  const [mediaFiles, setMediaFiles] = useState<MediaMetadata[]>([]);
  const [pageLoading, setPageLoading] = useState(true);
  const [prefixFilter, setPrefixFilter] = useState('');
  const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set());
  const [isUploadModalOpen, setIsUploadModalOpen] = useState(false);
  const [previewFile, setPreviewFile] = useState<MediaMetadata | null>(null);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [fileToDelete, setFileToDelete] = useState<MediaMetadata | null>(null);
  const [isBatchDeleteModalOpen, setIsBatchDeleteModalOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [currentPage, setCurrentPage] = useState(1);
  const itemsPerPage = 24;

  const loadMediaFiles = async () => {
    try {
      setPageLoading(true);
      const url = prefixFilter
        ? getApiUrl(`/media?prefix=${encodeURIComponent(prefixFilter)}`)
        : getApiUrl('/media');

      const response = await authenticatedFetch(
        url,
        token,
        { cache: 'no-store' },
        keycloak || undefined
      );

      if (!response.ok) {
        throw new Error('Failed to load media files');
      }

      const res = await response.json();
      setMediaFiles(Array.isArray(res.data) ? res.data : []);
    } catch (error) {
      console.error('Failed to load media files:', error);
      toast.error('Failed to load media files');
      setMediaFiles([]);
    } finally {
      setPageLoading(false);
    }
  };

  useEffect(() => {
    loadMediaFiles();
  }, [token, keycloak]);

  // Filtering
  const filteredMediaFiles = useMemo(() => {
    if (!prefixFilter) return mediaFiles;
    return mediaFiles.filter((file) =>
      file.path.toLowerCase().includes(prefixFilter.toLowerCase())
    );
  }, [mediaFiles, prefixFilter]);

  // Pagination
  const totalPages = Math.ceil(filteredMediaFiles.length / itemsPerPage);
  const paginatedFiles = useMemo(() => {
    const startIndex = (currentPage - 1) * itemsPerPage;
    return filteredMediaFiles.slice(startIndex, startIndex + itemsPerPage);
  }, [filteredMediaFiles, currentPage, itemsPerPage]);

  // Reset to page 1 when filter changes
  useEffect(() => {
    setCurrentPage(1);
    setSelectedFiles(new Set());
  }, [prefixFilter]);

  // Selection handlers
  const handleSelectFile = (path: string) => {
    setSelectedFiles((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(path)) {
        newSet.delete(path);
      } else {
        newSet.add(path);
      }
      return newSet;
    });
  };

  const handleSelectAll = () => {
    if (selectedFiles.size === paginatedFiles.length) {
      setSelectedFiles(new Set());
    } else {
      setSelectedFiles(new Set(paginatedFiles.map((f) => f.path)));
    }
  };

  const handleCopyUrl = (url: string) => {
    navigator.clipboard.writeText(url);
    toast.success('URL copied to clipboard');
  };

  // Delete handlers
  const handleDeleteClick = (media: MediaMetadata) => {
    setFileToDelete(media);
    setIsDeleteModalOpen(true);
  };

  const confirmDeleteSingle = async () => {
    if (!fileToDelete) return;

    try {
      setIsDeleting(true);
      const response = await authenticatedFetch(
        getApiUrl(`/media/delete/${encodeURIComponent(fileToDelete.path)}`),
        token,
        { method: 'DELETE' },
        keycloak || undefined
      );

      if (response.ok) {
        setMediaFiles((prev) => prev.filter((f) => f.path !== fileToDelete.path));
        setSelectedFiles((prev) => {
          const newSet = new Set(prev);
          newSet.delete(fileToDelete.path);
          return newSet;
        });
        setIsDeleteModalOpen(false);
        setFileToDelete(null);
        setPreviewFile(null);
        toast.success('File deleted successfully');
      } else {
        toast.error('Failed to delete file');
      }
    } catch (error) {
      console.error('Error deleting file:', error);
      toast.error('An error occurred while deleting the file');
    } finally {
      setIsDeleting(false);
    }
  };

  const confirmDeleteBatch = async () => {
    if (selectedFiles.size === 0) return;

    try {
      setIsDeleting(true);
      const pathsToDelete = Array.from(selectedFiles);

      const response = await authenticatedFetch(
        getApiUrl('/media'),
        token,
        {
          method: 'DELETE',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(pathsToDelete),
        },
        keycloak || undefined
      );

      if (response.ok) {
        const result = await response.json();
        setMediaFiles((prev) => prev.filter((f) => !selectedFiles.has(f.path)));
        setSelectedFiles(new Set());
        setIsBatchDeleteModalOpen(false);
        toast.success(`${result.data?.deletedCount || pathsToDelete.length} file(s) deleted successfully`);
      } else {
        toast.error('Failed to delete files');
      }
    } catch (error) {
      console.error('Error deleting files:', error);
      toast.error('An error occurred while deleting files');
    } finally {
      setIsDeleting(false);
    }
  };

  const cancelDelete = () => {
    setIsDeleteModalOpen(false);
    setIsBatchDeleteModalOpen(false);
    setFileToDelete(null);
  };

  const clearFilter = () => {
    setPrefixFilter('');
  };

  const isAllSelected = paginatedFiles.length > 0 && selectedFiles.size === paginatedFiles.length;

  // Grid skeleton component
  const GridSkeleton = () => (
    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
      {Array.from({ length: 12 }).map((_, i) => (
        <div key={i} className="card bg-base-100 shadow-sm">
          <div className="aspect-square bg-base-200 rounded-t-xl skeleton" />
          <div className="p-2 space-y-2">
            <div className="skeleton h-4 w-3/4" />
            <div className="skeleton h-3 w-1/2" />
          </div>
        </div>
      ))}
    </div>
  );

  return (
    <div className="space-y-6">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Media' },
        ]}
      />

      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold">Media</h1>
          <p className="text-base-content/60 text-sm">
            Manage your images and documents
          </p>
        </div>
        <div className="flex gap-2">
          <button
            className="btn btn-ghost btn-sm gap-2"
            onClick={loadMediaFiles}
            disabled={pageLoading}
          >
            <RefreshCw className={`w-4 h-4 ${pageLoading ? 'animate-spin' : ''}`} />
            Refresh
          </button>
          <button
            className="btn btn-primary gap-2"
            onClick={() => setIsUploadModalOpen(true)}
          >
            <Plus className="w-4 h-4" />
            Upload
          </button>
        </div>
      </div>

      {/* Filter and Actions Bar */}
      <div className="flex flex-col sm:flex-row gap-3 items-start sm:items-center justify-between">
        <div className="flex gap-3 flex-1">
          <label className="input input-bordered flex items-center gap-2 flex-1 max-w-md">
            <Search className="w-4 h-4 opacity-50" />
            <input
              type="text"
              placeholder="Filter by path..."
              className="grow"
              value={prefixFilter}
              onChange={(e) => setPrefixFilter(e.target.value)}
            />
            {prefixFilter && (
              <button
                type="button"
                className="btn btn-ghost btn-xs btn-circle"
                onClick={clearFilter}
              >
                <X className="w-3 h-3" />
              </button>
            )}
          </label>

          {prefixFilter && (
            <span className="badge badge-primary badge-lg">
              {filteredMediaFiles.length} / {mediaFiles.length}
            </span>
          )}
        </div>

        {/* Batch Actions */}
        {paginatedFiles.length > 0 && (
          <div className="flex gap-2 items-center">
            <button
              className="btn btn-ghost btn-sm gap-2"
              onClick={handleSelectAll}
            >
              {isAllSelected ? (
                <CheckSquare className="w-4 h-4" />
              ) : (
                <Square className="w-4 h-4" />
              )}
              {isAllSelected ? 'Deselect All' : 'Select All'}
            </button>

            {selectedFiles.size > 0 && (
              <button
                className="btn btn-error btn-sm btn-outline gap-2"
                onClick={() => setIsBatchDeleteModalOpen(true)}
              >
                <Trash2 className="w-4 h-4" />
                Delete ({selectedFiles.size})
              </button>
            )}
          </div>
        )}
      </div>

      {/* Media Grid */}
      <div className="card bg-base-100 shadow-sm">
        <div className="card-body">
          {pageLoading ? (
            <GridSkeleton />
          ) : paginatedFiles.length === 0 ? (
            /* Empty State */
            <div className="flex flex-col items-center justify-center py-16 px-4">
              <div className="bg-base-200 rounded-full p-4 mb-4">
                <FolderOpen className="w-8 h-8 text-base-content/40" />
              </div>
              <h3 className="text-lg font-semibold mb-1">
                {prefixFilter ? 'No media found' : 'No media yet'}
              </h3>
              <p className="text-base-content/60 text-center mb-4">
                {prefixFilter
                  ? 'Try adjusting your filter'
                  : 'Get started by uploading your first file'}
              </p>
              {prefixFilter ? (
                <button className="btn btn-ghost btn-sm" onClick={clearFilter}>
                  Clear filter
                </button>
              ) : (
                <button
                  className="btn btn-primary btn-sm gap-2"
                  onClick={() => setIsUploadModalOpen(true)}
                >
                  <Plus className="w-4 h-4" />
                  Upload Files
                </button>
              )}
            </div>
          ) : (
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
              {paginatedFiles.map((media) => (
                <MediaGridItem
                  key={media.path}
                  media={media}
                  isSelected={selectedFiles.has(media.path)}
                  onSelect={handleSelectFile}
                  onPreview={setPreviewFile}
                  onCopyUrl={handleCopyUrl}
                  onDelete={handleDeleteClick}
                />
              ))}
            </div>
          )}
        </div>

        {/* Pagination */}
        {!pageLoading && totalPages > 1 && (
          <div className="card-body border-t border-base-200 py-3">
            <div className="flex flex-col sm:flex-row items-center justify-between gap-3">
              <span className="text-sm text-base-content/60">
                Showing {(currentPage - 1) * itemsPerPage + 1} to{' '}
                {Math.min(currentPage * itemsPerPage, filteredMediaFiles.length)} of{' '}
                {filteredMediaFiles.length} files
              </span>
              <div className="join">
                <button
                  className="join-item btn btn-sm"
                  disabled={currentPage === 1}
                  onClick={() => setCurrentPage((p) => p - 1)}
                >
                  Previous
                </button>
                {Array.from({ length: totalPages }, (_, i) => i + 1)
                  .filter((page) => {
                    if (totalPages <= 5) return true;
                    if (page === 1 || page === totalPages) return true;
                    if (Math.abs(page - currentPage) <= 1) return true;
                    return false;
                  })
                  .map((page, index, arr) => (
                    <span key={page}>
                      {index > 0 && arr[index - 1] !== page - 1 && (
                        <button className="join-item btn btn-sm btn-disabled">...</button>
                      )}
                      <button
                        className={`join-item btn btn-sm ${currentPage === page ? 'btn-active' : ''}`}
                        onClick={() => setCurrentPage(page)}
                      >
                        {page}
                      </button>
                    </span>
                  ))}
                <button
                  className="join-item btn btn-sm"
                  disabled={currentPage === totalPages}
                  onClick={() => setCurrentPage((p) => p + 1)}
                >
                  Next
                </button>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Upload Modal */}
      <MediaUploadModal
        isOpen={isUploadModalOpen}
        onClose={() => setIsUploadModalOpen(false)}
        onUploadComplete={loadMediaFiles}
      />

      {/* Preview Modal */}
      <MediaPreviewModal
        media={previewFile}
        isOpen={previewFile !== null}
        onClose={() => setPreviewFile(null)}
        onDelete={handleDeleteClick}
      />

      {/* Single Delete Confirmation Modal */}
      <dialog className={`modal ${isDeleteModalOpen ? 'modal-open' : ''}`}>
        <div className="modal-box">
          <h3 className="font-bold text-lg">Confirm Deletion</h3>
          <p className="py-4">
            Are you sure you want to delete{' '}
            <span className="font-semibold">"{fileToDelete ? getFileName(fileToDelete.path) : ''}"</span>?
            This action cannot be undone.
          </p>
          <div className="modal-action">
            <button className="btn btn-ghost" onClick={cancelDelete} disabled={isDeleting}>
              Cancel
            </button>
            <button className="btn btn-error" onClick={confirmDeleteSingle} disabled={isDeleting}>
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
        <form method="dialog" className="modal-backdrop" onClick={cancelDelete}>
          <button>close</button>
        </form>
      </dialog>

      {/* Batch Delete Confirmation Modal */}
      <dialog className={`modal ${isBatchDeleteModalOpen ? 'modal-open' : ''}`}>
        <div className="modal-box">
          <h3 className="font-bold text-lg">Confirm Batch Deletion</h3>
          <p className="py-4">
            Are you sure you want to delete{' '}
            <span className="font-semibold">{selectedFiles.size} file(s)</span>?
            This action cannot be undone.
          </p>
          <div className="modal-action">
            <button className="btn btn-ghost" onClick={cancelDelete} disabled={isDeleting}>
              Cancel
            </button>
            <button className="btn btn-error" onClick={confirmDeleteBatch} disabled={isDeleting}>
              {isDeleting ? (
                <>
                  <span className="loading loading-spinner loading-sm"></span>
                  Deleting...
                </>
              ) : (
                `Delete ${selectedFiles.size} Files`
              )}
            </button>
          </div>
        </div>
        <form method="dialog" className="modal-backdrop" onClick={cancelDelete}>
          <button>close</button>
        </form>
      </dialog>
    </div>
  );
}
