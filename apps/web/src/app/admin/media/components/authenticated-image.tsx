import React, { useEffect, useState } from 'react';
import { authenticatedFetch } from '@/config/api.config';

interface AuthenticatedImageProps
  extends Omit<React.ImgHTMLAttributes<HTMLImageElement>, 'src'> {
  src: string;
  token: string | null;
}

const AuthenticatedImage: React.FC<AuthenticatedImageProps> = ({ src, token, ...props }) => {
  const [objectUrl, setObjectUrl] = useState<string | null>(null);

  useEffect(() => {
    let isCancelled = false;
    let currentObjectUrl: string | null = null;

    setObjectUrl(null);

    const loadImage = async () => {
      try {
        const response = await authenticatedFetch(src, token, { cache: 'no-store' });
        if (!response.ok) {
          throw new Error(`Image request failed with status ${response.status}`);
        }

        const blob = await response.blob();
        const nextObjectUrl = URL.createObjectURL(blob);
        if (isCancelled) {
          URL.revokeObjectURL(nextObjectUrl);
          return;
        }

        currentObjectUrl = nextObjectUrl;
        setObjectUrl(nextObjectUrl);
      } catch {
        if (!isCancelled) {
          setObjectUrl(null);
        }
      }
    };

    void loadImage();

    return () => {
      isCancelled = true;
      if (currentObjectUrl) {
        URL.revokeObjectURL(currentObjectUrl);
      }
    };
  }, [src, token]);

  if (!objectUrl) {
    return (
      <div
        data-testid="authenticated-image-placeholder"
        aria-busy="true"
        className={props.className}
      />
    );
  }

  return <img {...props} src={objectUrl} />;
};

export default AuthenticatedImage;
