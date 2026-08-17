interface EditorialImageProps {
  src: string;
  alt: string;
  aspect?: string;
  fit?: 'cover' | 'natural';
  className?: string;
}

const EditorialImage = ({ src, alt, aspect, fit = 'cover', className = '' }: EditorialImageProps) => (
  <figure
    className={`editorial-image ${fit === 'natural' ? 'editorial-image--natural' : ''} ${className}`.trim()}
    style={aspect ? { aspectRatio: aspect } : undefined}
  >
    <img src={src} alt={alt} />
  </figure>
);

export default EditorialImage;
