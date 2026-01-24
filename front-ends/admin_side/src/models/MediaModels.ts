/**
 * Media file metadata returned from the API
 */
export interface MediaMetadata {
  path: string;
  url: string;
  contentType: string;
  size: number;
  lastModified: string;
}

/**
 * Response from uploading a media file
 */
export interface MediaUploadResponse {
  path: string;
  url: string;
}

/**
 * Response from batch delete operation
 */
export interface MediaDeleteBatchResponse {
  deletedCount: number;
}

/**
 * Helper to check if a content type is an image
 */
export const isImageContentType = (contentType: string): boolean => {
  return contentType.startsWith('image/');
};

/**
 * Helper to format file size in human-readable format
 */
export const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
};

/**
 * Get file extension from path
 */
export const getFileExtension = (path: string): string => {
  const lastDot = path.lastIndexOf('.');
  return lastDot !== -1 ? path.substring(lastDot + 1).toLowerCase() : '';
};

/**
 * Get filename from path
 */
export const getFileName = (path: string): string => {
  const lastSlash = path.lastIndexOf('/');
  return lastSlash !== -1 ? path.substring(lastSlash + 1) : path;
};
