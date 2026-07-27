interface EditorialImageProps {
  src: string;
  alt: string;
  aspect?: string;
  className?: string;
}

const EditorialImage = ({ src, alt, aspect = '4 / 3', className = '' }: EditorialImageProps) => (
  <figure className={`editorial-image ${className}`.trim()} style={{ aspectRatio: aspect }}>
    <img src={src} alt={alt} />
  </figure>
);

export default EditorialImage;
