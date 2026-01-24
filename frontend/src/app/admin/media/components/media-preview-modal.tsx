import React from 'react';
import { X, Copy, Trash2, ExternalLink, FileText, FileSpreadsheet, File } from 'lucide-react';
import { toast } from 'sonner';
import type { MediaMetadata } from '@/models/MediaModels';
import { isImageContentType, formatFileSize, getFileName, getFileExtension } from '@/models/MediaModels';
import { getMediaImageUrl } from '@/config/api.config';

interface MediaPreviewModalProps {
  media: MediaMetadata | null;
  isOpen: boolean;
  onClose: () => void;
  onDelete: (media: MediaMetadata) => void;
}

const getFileIcon = (_contentType: string, extension: string) => {
  switch (extension) {
    case 'pdf':
      return <FileText className="w-24 h-24 text-error" />;
    case 'doc':
    case 'docx':
      return <FileText className="w-24 h-24 text-info" />;
    case 'xls':
    case 'xlsx':
    case 'csv':
      return <FileSpreadsheet className="w-24 h-24 text-success" />;
    case 'md':
    case 'txt':
      return <File className="w-24 h-24 text-base-content" />;
    default:
      return <File className="w-24 h-24 text-base-content/60" />;
  }
};

const MediaPreviewModal: React.FC<MediaPreviewModalProps> = ({
  media,
  isOpen,
  onClose,
  onDelete,
}) => {
  if (!isOpen || !media) return null;

  const fileName = getFileName(media.path);
  const extension = getFileExtension(media.path);
  const isImage = isImageContentType(media.contentType);
  const fullImageUrl = isImage ? getMediaImageUrl(media.path) : null;

  const handleCopyUrl = () => {
    navigator.clipboard.writeText(media.url);
    toast.success('URL copied to clipboard');
  };

  const handleOpenInNewTab = () => {
    window.open(media.url, '_blank');
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString();
  };

  return (
    <div className="modal modal-open">
      <div className="modal-box max-w-4xl">
        <button
          className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
          onClick={onClose}
        >
          <X className="w-4 h-4" />
        </button>

        <h3 className="font-bold text-lg mb-4 pr-8 truncate" title={fileName}>
          {fileName}
        </h3>

        <div className="flex flex-col lg:flex-row gap-6">
          {/* Preview */}
          <div className="flex-1 flex items-center justify-center bg-base-200 rounded-lg p-4 min-h-[300px]">
            {isImage && fullImageUrl ? (
              <img
                src={fullImageUrl}
                alt={fileName}
                className="max-w-full max-h-[400px] object-contain rounded"
              />
            ) : (
              <div className="text-center">
                {getFileIcon(media.contentType, extension)}
                <p className="mt-4 text-base-content/60">Preview not available</p>
              </div>
            )}
          </div>

          {/* Metadata */}
          <div className="lg:w-72 space-y-4">
            <div>
              <label className="text-xs text-base-content/60 uppercase tracking-wide">
                File Name
              </label>
              <p className="text-sm font-medium break-all">{fileName}</p>
            </div>

            <div>
              <label className="text-xs text-base-content/60 uppercase tracking-wide">
                Path
              </label>
              <p className="text-sm break-all text-base-content/80">{media.path}</p>
            </div>

            <div>
              <label className="text-xs text-base-content/60 uppercase tracking-wide">
                Content Type
              </label>
              <p className="text-sm">{media.contentType}</p>
            </div>

            <div>
              <label className="text-xs text-base-content/60 uppercase tracking-wide">
                Size
              </label>
              <p className="text-sm">{formatFileSize(media.size)}</p>
            </div>

            <div>
              <label className="text-xs text-base-content/60 uppercase tracking-wide">
                Last Modified
              </label>
              <p className="text-sm">{formatDate(media.lastModified)}</p>
            </div>

            <div>
              <label className="text-xs text-base-content/60 uppercase tracking-wide">
                URL
              </label>
              <div className="flex items-center gap-2 mt-1">
                <input
                  type="text"
                  readOnly
                  value={media.url}
                  className="input input-sm input-bordered flex-1 text-xs"
                />
                <button
                  className="btn btn-sm btn-ghost btn-square"
                  onClick={handleCopyUrl}
                  title="Copy URL"
                >
                  <Copy className="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* Actions */}
        <div className="modal-action">
          <button className="btn btn-ghost" onClick={handleOpenInNewTab}>
            <ExternalLink className="w-4 h-4" />
            Open in New Tab
          </button>
          <button className="btn btn-ghost" onClick={handleCopyUrl}>
            <Copy className="w-4 h-4" />
            Copy URL
          </button>
          <button
            className="btn btn-error btn-outline"
            onClick={() => onDelete(media)}
          >
            <Trash2 className="w-4 h-4" />
            Delete
          </button>
        </div>
      </div>
      <div className="modal-backdrop bg-black/50" onClick={onClose} />
    </div>
  );
};

export default MediaPreviewModal;
