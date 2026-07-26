interface ContentSkeletonProps {
  variant?: 'home' | 'categories' | 'category' | 'post';
}

const ContentSkeleton = ({ variant = 'home' }: ContentSkeletonProps) => (
  <div className={`content-skeleton content-skeleton--${variant}`} aria-busy="true" aria-label="Loading">
    <span className="skeleton-block skeleton-block--wide" />
    <span className="skeleton-block skeleton-block--title" />
    <span className="skeleton-block skeleton-block--copy" />
    <span className="skeleton-block skeleton-block--copy" />
  </div>
);

export default ContentSkeleton;
