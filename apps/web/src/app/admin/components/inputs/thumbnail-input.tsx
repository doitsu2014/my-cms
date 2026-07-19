import React, { useState, useEffect } from 'react';
import { toast } from 'sonner';
import { getMediaUploadApiUrl, createAuthHeaders } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';

interface ThumbnailsInputProps {
  onUploadSuccess: (urls: string[]) => void;
  value?: string[];
}

const ThumbnailsInput: React.FC<ThumbnailsInputProps> = ({
  onUploadSuccess,
  value = []
}) => {
  const { token } = useAuth();
  const [thumbnails, setThumbnails] = useState<string[]>([]);
  const [isUploading, setIsUploading] = useState(false);
  const [thumbnailUrl, setThumbnailUrl] = useState('');

  useEffect(() => {
    setThumbnails(value);
  }, [value]);

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (!files || files.length === 0) return;

    setIsUploading(true);
    try {
      const uploadedUrls: string[] = [];
      for (const file of Array.from(files)) {
        const formData = new FormData();
        formData.append('image', file);

        const response = await fetch(getMediaUploadApiUrl(), {
          method: 'POST',
          headers: createAuthHeaders(token),
          body: formData
        });

        if (!response.ok) {
          throw new Error('Image upload failed');
        }

        const data = await response.json();
        uploadedUrls.push(data.data.url);
      }

      const newThumbnails = [...thumbnails, ...uploadedUrls];
      setThumbnails(newThumbnails);
      onUploadSuccess(newThumbnails);
    } catch (error) {
      console.error('Error uploading images:', error);
      toast.error('Image upload failed. Please try again.');
    } finally {
      setIsUploading(false);
    }
  };

  const handleUrlSubmit = () => {
    if (thumbnailUrl.trim()) {
      const newThumbnails = [...thumbnails, thumbnailUrl.trim()];
      setThumbnails(newThumbnails);
      onUploadSuccess(newThumbnails);
      setThumbnailUrl('');
    }
  };

  const handleRemoveThumbnail = (index: number) => {
    const updatedThumbnails = thumbnails.filter((_, i) => i !== index);
    setThumbnails(updatedThumbnails);
    onUploadSuccess(updatedThumbnails);
  };

  return (
    <div className="form-control w-full max-w-xs">
      <div className="flex flex-wrap gap-4">
        {thumbnails.map((thumbnail, index) => (
          <div key={index} className="relative group">
            <div className="avatar">
              <div className="w-24 h-24 rounded border border-base-300">
                <img src={thumbnail} alt={`Thumbnail ${index + 1}`} />
              </div>
            </div>
            <button
              type="button"
              className="absolute top-0 right-0 bg-error text-white rounded-full w-6 h-6 flex items-center justify-center opacity-0 group-hover:opacity-100"
              onClick={() => handleRemoveThumbnail(index)}>
              ✕
            </button>
          </div>
        ))}
        {!isUploading && (
          <div className="avatar placeholder">
            <div className="w-24 h-24 bg-neutral-focus text-neutral-content rounded border border-base-300 flex items-center justify-center">
              <span className="text-sm text-base-content/70">No Image</span>
            </div>
          </div>
        )}
      </div>
      <input
        type="file"
        accept="image/*"
        multiple
        className="file-input file-input-bordered file-input-primary w-full max-w-xs mt-4"
        onChange={handleFileChange}
        disabled={isUploading}
      />
      {isUploading && <span className="text-sm text-info mt-2">Uploading...</span>}
      <div className="mt-4">
        <label className="label">
          <span className="label-text">Or enter a URL</span>
        </label>
        <div className="flex space-x-2">
          <input
            type="url"
            value={thumbnailUrl}
            onChange={(e) => setThumbnailUrl(e.target.value)}
            className="input input-bordered w-full"
            placeholder="https://example.com/image.jpg"
          />
          <button
            type="button"
            className="btn btn-primary"
            onClick={handleUrlSubmit}
            disabled={!thumbnailUrl.trim()}>
            Add
          </button>
        </div>
      </div>
    </div>
  );
};

export default ThumbnailsInput;
