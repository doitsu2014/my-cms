import { useCallback, useEffect, useRef, useState } from 'react';
import { toast } from 'sonner';

import { useAuth } from '@/auth/AuthContext';
import { authenticatedFetch, createAuthHeaders, getApiUrl, getMediaUploadApiUrl } from '@/config/api.config';
import { getFileName, isImageContentType, type MediaMetadata } from '@/models/MediaModels';

export interface ImageInsertionDialogProps {
  isOpen: boolean;
  onInsert: (urls: string[]) => void;
  onClose: () => void;
  restoreFocusTo?: HTMLElement | null;
}

type QueryState = 'idle' | 'loading' | 'ready' | 'error';

export function ImageInsertionDialog({ isOpen, onInsert, onClose, restoreFocusTo }: ImageInsertionDialogProps) {
  const { token } = useAuth();
  const [queryState, setQueryState] = useState<QueryState>('idle');
  const [images, setImages] = useState<MediaMetadata[]>([]);
  const [selected, setSelected] = useState<MediaMetadata[]>([]);
  const [isUploading, setIsUploading] = useState(false);
  const [uploadFeedback, setUploadFeedback] = useState('');
  const dialogRef = useRef<HTMLDivElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const restoreFocus = useCallback(() => {
    requestAnimationFrame(() => restoreFocusTo?.focus());
  }, [restoreFocusTo]);

  const close = useCallback(() => {
    setSelected([]);
    setUploadFeedback('');
    onClose();
    restoreFocus();
  }, [onClose, restoreFocus]);

  const loadMedia = useCallback(async () => {
    setQueryState('loading');
    try {
      const response = await authenticatedFetch(getApiUrl('/media'), token, { cache: 'no-store' });
      if (!response.ok) throw new Error('Media library request failed');
      const body = await response.json() as { data?: MediaMetadata[] };
      setImages((body.data ?? []).filter((media) => isImageContentType(media.contentType)));
      setSelected([]);
      setQueryState('ready');
    } catch (error) {
      console.error('Unable to load media library', error);
      setQueryState('error');
    }
  }, [token]);

  useEffect(() => {
    if (!isOpen) return;
    void loadMedia();
    const frame = requestAnimationFrame(() => dialogRef.current?.focus());
    return () => cancelAnimationFrame(frame);
  }, [isOpen, loadMedia]);

  useEffect(() => {
    if (!isOpen) return;
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') close();
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [close, isOpen]);

  const toggleSelection = useCallback((media: MediaMetadata) => {
    setSelected((current) => current.some((item) => item.path === media.path)
      ? current.filter((item) => item.path !== media.path)
      : [...current, media]);
  }, []);

  const insertSelection = useCallback(() => {
    if (selected.length === 0 || queryState !== 'ready') return;
    onInsert(selected.map((media) => media.url));
  }, [onInsert, queryState, selected]);

  const uploadFiles = useCallback(async (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(event.target.files ?? []);
    if (files.length === 0) return;

    setIsUploading(true);
    setUploadFeedback('');
    const urls: string[] = [];
    let failures = 0;
    for (const file of files) {
      try {
        const formData = new FormData();
        formData.append('image', file);
        const response = await fetch(getMediaUploadApiUrl(), {
          method: 'POST',
          headers: createAuthHeaders(token),
          body: formData,
        });
        if (!response.ok) throw new Error('Image upload failed');
        const body = await response.json() as { data?: { url?: string } };
        if (!body.data?.url) throw new Error('Image upload returned no URL');
        urls.push(body.data.url);
      } catch (error) {
        failures += 1;
        console.error('Unable to upload image', error);
      }
    }

    fileInputRef.current && (fileInputRef.current.value = '');
    setIsUploading(false);
    const message = urls.length > 0
      ? `${urls.length} image${urls.length === 1 ? '' : 's'} added, ${failures} failed.`
      : `No images added, ${failures} failed.`;
    setUploadFeedback(message);
    if (urls.length > 0) {
      onInsert(urls);
      toast.success(message);
    } else {
      toast.error(message);
    }
  }, [onInsert, token]);

  if (!isOpen) return null;

  return (
    <div className="modal modal-open" role="dialog" aria-modal="true" aria-labelledby="image-insertion-title">
      <div ref={dialogRef} className="modal-box max-w-3xl" tabIndex={-1}>
        <div className="flex items-center justify-between gap-4">
          <h2 id="image-insertion-title" className="text-lg font-bold">Insert images</h2>
          <button type="button" className="btn btn-sm btn-circle btn-ghost" aria-label="Close image insertion" onClick={close}>✕</button>
        </div>

        <label className="mt-4 block text-sm font-medium" htmlFor="rich-text-image-upload">Upload local images</label>
        <input
          ref={fileInputRef}
          id="rich-text-image-upload"
          type="file"
          accept="image/*"
          multiple
          className="file-input file-input-bordered mt-1 w-full"
          disabled={isUploading}
          onChange={uploadFiles}
        />
        {isUploading && <p className="mt-2 text-sm" aria-live="polite">Uploading images…</p>}
        {uploadFeedback && <p className="mt-2 text-sm" aria-live="polite">{uploadFeedback}</p>}

        <div className="divider">or choose from media library</div>
        {queryState === 'loading' && <p aria-live="polite">Loading images…</p>}
        {queryState === 'error' && (
          <div className="alert alert-error">
            <span>Unable to load the media library.</span>
            <button type="button" className="btn btn-sm" onClick={() => void loadMedia()}>Retry</button>
          </div>
        )}
        {queryState === 'ready' && images.length === 0 && <p>No images are available in the media library.</p>}
        {queryState === 'ready' && images.length > 0 && (
          <div className="mt-3 grid max-h-72 grid-cols-2 gap-3 overflow-y-auto sm:grid-cols-3">
            {images.map((media) => {
              const ordinal = selected.findIndex((item) => item.path === media.path);
              return (
                <button
                  key={media.path}
                  type="button"
                  className={`relative rounded border p-2 text-left ${ordinal >= 0 ? 'border-primary ring-2 ring-primary' : 'border-base-300'}`}
                  aria-pressed={ordinal >= 0}
                  aria-label={getFileName(media.path)}
                  onClick={() => toggleSelection(media)}
                >
                  <img src={media.url} alt="" className="h-24 w-full rounded object-cover" />
                  <span className="mt-1 block truncate text-xs">{getFileName(media.path)}</span>
                  {ordinal >= 0 && <span className="badge badge-primary absolute right-2 top-2" aria-label={`Selection ${ordinal + 1}`}>{ordinal + 1}</span>}
                </button>
              );
            })}
          </div>
        )}

        <div className="modal-action">
          <button type="button" className="btn btn-ghost" onClick={close}>Cancel</button>
          <button type="button" className="btn btn-primary" disabled={queryState !== 'ready' || selected.length === 0} onClick={insertSelection}>Insert selected images</button>
        </div>
      </div>
      <div className="modal-backdrop" onClick={close} />
    </div>
  );
}
