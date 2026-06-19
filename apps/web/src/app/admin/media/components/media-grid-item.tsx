import React from 'react';
import { FileText, FileSpreadsheet, File, Image, Copy, Trash2, Eye } from 'lucide-react';
import type { MediaMetadata } from '@/models/MediaModels';
import { isImageContentType, formatFileSize, getFileName, getFileExtension } from '@/models/MediaModels';
import { getMediaImageUrl } from '@/config/api.config';

interface MediaGridItemProps {
  media: MediaMetadata;
  isSelected: boolean;
  onSelect: (path: string) => void;
  onPreview: (media: MediaMetadata) => void;
  onCopyUrl: (url: string) => void;
  onDelete: (media: MediaMetadata) => void;
}

const getFileIcon = (contentType: string, extension: string) => {
  if (isImageContentType(contentType)) {
    return <Image className="w-12 h-12 text-primary" />;
  }

  switch (extension) {
    case 'pdf':
      return <FileText className="w-12 h-12 text-error" />;
    case 'doc':
    case 'docx':
      return <FileText className="w-12 h-12 text-info" />;
    case 'xls':
    case 'xlsx':
    case 'csv':
      return <FileSpreadsheet className="w-12 h-12 text-success" />;
    case 'md':
    case 'txt':
      return <File className="w-12 h-12 text-base-content" />;
    default:
      return <File className="w-12 h-12 text-base-content/60" />;
  }
};

const MediaGridItem: React.FC<MediaGridItemProps> = ({
  media,
  isSelected,
  onSelect,
  onPreview,
  onCopyUrl,
  onDelete,
}) => {
  const fileName = getFileName(media.path);
  const extension = getFileExtension(media.path);
  const isImage = isImageContentType(media.contentType);
  const thumbnailUrl = isImage ? getMediaImageUrl(media.path) + '?w=200&h=200' : null;

  return (
    <div
      className={`card bg-base-100 shadow-sm border-2 transition-all hover:shadow-md cursor-pointer group ${
        isSelected ? 'border-primary' : 'border-transparent hover:border-base-300'
      }`}
    >
      {/* Checkbox for selection */}
      <div className="absolute top-2 left-2 z-10">
        <input
          type="checkbox"
          className="checkbox checkbox-primary checkbox-sm"
          checked={isSelected}
          onChange={() => onSelect(media.path)}
          onClick={(e) => e.stopPropagation()}
        />
      </div>

      {/* Action buttons */}
      <div className="absolute top-2 right-2 z-10 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
        <button
          className="btn btn-xs btn-circle btn-ghost bg-base-100/80"
          onClick={(e) => {
            e.stopPropagation();
            onPreview(media);
          }}
          title="Preview"
        >
          <Eye className="w-3 h-3" />
        </button>
        <button
          className="btn btn-xs btn-circle btn-ghost bg-base-100/80"
          onClick={(e) => {
            e.stopPropagation();
            onCopyUrl(media.url);
          }}
          title="Copy URL"
        >
          <Copy className="w-3 h-3" />
        </button>
        <button
          className="btn btn-xs btn-circle btn-ghost bg-base-100/80 hover:btn-error"
          onClick={(e) => {
            e.stopPropagation();
            onDelete(media);
          }}
          title="Delete"
        >
          <Trash2 className="w-3 h-3" />
        </button>
      </div>

      {/* Thumbnail / Icon */}
      <div
        className="aspect-square flex items-center justify-center bg-base-200 rounded-t-xl overflow-hidden"
        onClick={() => onPreview(media)}
      >
        {thumbnailUrl ? (
          <img
            src={thumbnailUrl}
            alt={fileName}
            className="w-full h-full object-cover"
            loading="lazy"
          />
        ) : (
          getFileIcon(media.contentType, extension)
        )}
      </div>

      {/* File info */}
      <div className="p-2">
        <p className="text-sm font-medium truncate" title={fileName}>
          {fileName}
        </p>
        <p className="text-xs text-base-content/60">
          {formatFileSize(media.size)}
        </p>
      </div>
    </div>
  );
};

export default MediaGridItem;
