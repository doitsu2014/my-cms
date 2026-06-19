import React, { useState, useRef, useCallback } from 'react';
import { Upload, X, FileUp, CheckCircle, AlertCircle, Loader2 } from 'lucide-react';
import { toast } from 'sonner';
import { useAuth } from '@/auth/AuthContext';
import { getApiUrl, createAuthHeaders } from '@/config/api.config';

interface MediaUploadModalProps {
  isOpen: boolean;
  onClose: () => void;
  onUploadComplete: () => void;
}

interface UploadFile {
  file: File;
  status: 'pending' | 'uploading' | 'success' | 'error';
  progress: number;
  error?: string;
}

const MediaUploadModal: React.FC<MediaUploadModalProps> = ({
  isOpen,
  onClose,
  onUploadComplete,
}) => {
  const { token } = useAuth();
  const [files, setFiles] = useState<UploadFile[]>([]);
  const [isDragging, setIsDragging] = useState(false);
  const [isUploading, setIsUploading] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  }, []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    const droppedFiles = Array.from(e.dataTransfer.files);
    addFiles(droppedFiles);
  }, []);

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      const selectedFiles = Array.from(e.target.files);
      addFiles(selectedFiles);
    }
  };

  const addFiles = (newFiles: File[]) => {
    const uploadFiles: UploadFile[] = newFiles.map((file) => ({
      file,
      status: 'pending' as const,
      progress: 0,
    }));
    setFiles((prev) => [...prev, ...uploadFiles]);
  };

  const removeFile = (index: number) => {
    setFiles((prev) => prev.filter((_, i) => i !== index));
  };

  const uploadFile = async (uploadFile: UploadFile, index: number): Promise<void> => {
    const formData = new FormData();
    formData.append('file', uploadFile.file);

    setFiles((prev) =>
      prev.map((f, i) => (i === index ? { ...f, status: 'uploading' as const, progress: 0 } : f))
    );

    try {
      const response = await fetch(getApiUrl('/media'), {
        method: 'POST',
        headers: createAuthHeaders(token),
        body: formData,
      });

      if (!response.ok) {
        throw new Error(`Upload failed: ${response.statusText}`);
      }

      // Response contains { data: { path, url } } but we don't need to use it
      await response.json();

      setFiles((prev) =>
        prev.map((f, i) =>
          i === index ? { ...f, status: 'success' as const, progress: 100 } : f
        )
      );
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Upload failed';
      setFiles((prev) =>
        prev.map((f, i) =>
          i === index ? { ...f, status: 'error' as const, error: errorMessage } : f
        )
      );
    }
  };

  const handleUploadAll = async () => {
    const pendingFiles = files.filter((f) => f.status === 'pending' || f.status === 'error');
    if (pendingFiles.length === 0) {
      toast.info('No files to upload');
      return;
    }

    setIsUploading(true);

    // Upload files sequentially to avoid overwhelming the server
    for (let i = 0; i < files.length; i++) {
      if (files[i].status === 'pending' || files[i].status === 'error') {
        await uploadFile(files[i], i);
      }
    }

    setIsUploading(false);

    const successCount = files.filter((f) => f.status === 'success').length;
    const errorCount = files.filter((f) => f.status === 'error').length;

    if (errorCount === 0) {
      toast.success(`${successCount} file(s) uploaded successfully`);
      onUploadComplete();
      handleClose();
    } else {
      toast.error(`${errorCount} file(s) failed to upload`);
    }
  };

  const handleClose = () => {
    if (!isUploading) {
      setFiles([]);
      onClose();
    }
  };

  const pendingCount = files.filter((f) => f.status === 'pending').length;
  const successCount = files.filter((f) => f.status === 'success').length;

  if (!isOpen) return null;

  return (
    <div className="modal modal-open">
      <div className="modal-box max-w-2xl">
        <button
          className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
          onClick={handleClose}
          disabled={isUploading}
        >
          <X className="w-4 h-4" />
        </button>

        <h3 className="font-bold text-lg mb-4">Upload Media</h3>

        {/* Drop Zone */}
        <div
          className={`border-2 border-dashed rounded-lg p-8 text-center transition-colors ${
            isDragging
              ? 'border-primary bg-primary/10'
              : 'border-base-300 hover:border-primary/50'
          }`}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
        >
          <Upload className="w-12 h-12 mx-auto mb-4 text-base-content/40" />
          <p className="text-base-content/70 mb-2">
            Drag and drop files here, or{' '}
            <button
              className="link link-primary"
              onClick={() => fileInputRef.current?.click()}
              disabled={isUploading}
            >
              browse
            </button>
          </p>
          <p className="text-xs text-base-content/50">
            Supports images, PDFs, Word, Excel, and text files
          </p>
          <input
            ref={fileInputRef}
            type="file"
            className="hidden"
            multiple
            onChange={handleFileSelect}
            accept="image/*,.pdf,.doc,.docx,.xls,.xlsx,.txt,.csv,.md"
          />
        </div>

        {/* File List */}
        {files.length > 0 && (
          <div className="mt-4 max-h-60 overflow-y-auto">
            <div className="flex justify-between items-center mb-2">
              <span className="text-sm text-base-content/70">
                {files.length} file(s) selected
              </span>
              {successCount > 0 && (
                <span className="text-sm text-success">
                  {successCount} uploaded
                </span>
              )}
            </div>
            <ul className="space-y-2">
              {files.map((uploadFile, index) => (
                <li
                  key={index}
                  className="flex items-center gap-3 p-2 bg-base-200 rounded-lg"
                >
                  <FileUp className="w-5 h-5 text-base-content/60 flex-shrink-0" />
                  <div className="flex-1 min-w-0">
                    <p className="text-sm truncate">{uploadFile.file.name}</p>
                    <p className="text-xs text-base-content/50">
                      {(uploadFile.file.size / 1024).toFixed(1)} KB
                    </p>
                  </div>
                  <div className="flex-shrink-0">
                    {uploadFile.status === 'pending' && (
                      <button
                        className="btn btn-xs btn-ghost btn-circle"
                        onClick={() => removeFile(index)}
                        disabled={isUploading}
                      >
                        <X className="w-3 h-3" />
                      </button>
                    )}
                    {uploadFile.status === 'uploading' && (
                      <Loader2 className="w-4 h-4 animate-spin text-primary" />
                    )}
                    {uploadFile.status === 'success' && (
                      <CheckCircle className="w-4 h-4 text-success" />
                    )}
                    {uploadFile.status === 'error' && (
                      <div className="tooltip tooltip-left" data-tip={uploadFile.error}>
                        <AlertCircle className="w-4 h-4 text-error" />
                      </div>
                    )}
                  </div>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Actions */}
        <div className="modal-action">
          <button
            className="btn btn-ghost"
            onClick={handleClose}
            disabled={isUploading}
          >
            Cancel
          </button>
          <button
            className="btn btn-primary"
            onClick={handleUploadAll}
            disabled={isUploading || pendingCount === 0}
          >
            {isUploading ? (
              <>
                <Loader2 className="w-4 h-4 animate-spin" />
                Uploading...
              </>
            ) : (
              <>
                <Upload className="w-4 h-4" />
                Upload {pendingCount > 0 ? `(${pendingCount})` : ''}
              </>
            )}
          </button>
        </div>
      </div>
      <div className="modal-backdrop bg-black/50" onClick={handleClose} />
    </div>
  );
};

export default MediaUploadModal;
