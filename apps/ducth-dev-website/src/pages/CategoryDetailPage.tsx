import { useQuery } from '@apollo/client';
import { useTranslation } from 'react-i18next';
import { useParams } from 'react-router-dom';
import { GET_POSTS_BY_CATEGORY } from '../infrastructure/graphql/queries';
import { getMediaUrl } from '../config/get-media-url';
import { readBrowserConfig } from '../config/read-browser-config';

interface BlogPost {
  id: string;
  title: string;
  slug: string;
  previewContent: string;
  createdAt: string;
  thumbnailPaths?: string[];
  published: boolean;
  categories?: {
    displayName: string;
    slug: string;
  };
  postTranslations?: {
    nodes: Array<{
      languageCode: string;
      title: string;
      previewContent?: string;
    }>;
  };
}

interface Category {
  id: string;
  displayName: string;
  slug: string;
  categoryTranslations?: {
    nodes: Array<{
      languageCode: string;
      displayName: string;
    }>;
  };
}

const CategoryDetailPage = () => {
  const { t } = useTranslation();
  const { lang, slug } = useParams<{ lang: string; slug: string }>();
  const currentLang = lang || 'en';

  const { loading, error, data } = useQuery(GET_POSTS_BY_CATEGORY, {
    variables: { categorySlug: slug },
  });

  // Helper to format date
  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString(
      currentLang === 'vi' ? 'vi-VN' : 'en-US',
      { year: 'numeric', month: 'long', day: 'numeric' }
    );
  };

  // Get thumbnail URL
  const getThumbnailUrl = (post: BlogPost) => {
    if (post.thumbnailPaths && post.thumbnailPaths.length > 0) {
      const path = post.thumbnailPaths[0];
      // If path is already a full URL, return it directly
      if (path.startsWith('http://') || path.startsWith('https://')) {
        return path;
      }
      return getMediaUrl(path, readBrowserConfig().mediaBaseUrl);
    }
    return undefined;
  };

  // Get translated category name
  const getTranslatedCategoryName = (category: Category) => {
    if (currentLang !== 'en' && category.categoryTranslations?.nodes) {
      const translation = category.categoryTranslations.nodes.find(
        (t) => t.languageCode === currentLang
      );
      if (translation?.displayName) return translation.displayName;
    }
    return category.displayName;
  };

  // Get translated post title
  const getTranslatedTitle = (post: BlogPost) => {
    if (currentLang !== 'en' && post.postTranslations?.nodes) {
      const translation = post.postTranslations.nodes.find(
        (t) => t.languageCode === currentLang
      );
      if (translation?.title) return translation.title;
    }
    return post.title;
  };

  // Get translated preview
  const getTranslatedPreview = (post: BlogPost) => {
    if (currentLang !== 'en' && post.postTranslations?.nodes) {
      const translation = post.postTranslations.nodes.find(
        (t) => t.languageCode === currentLang
      );
      if (translation?.previewContent) return translation.previewContent;
    }
    return post.previewContent;
  };

  const category = data?.categories?.nodes?.[0];
  const categoryName = category ? getTranslatedCategoryName(category) : '';
  
  // Filter published posts for this category and sort by date (latest first)
  const posts = (data?.posts?.nodes?.filter((post: BlogPost) => 
    post.published && post.categories?.slug === slug
  ) || [])
    .sort((a: BlogPost, b: BlogPost) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());

  return (
    <div className="space-y-8">
      {/* Breadcrumb */}
      <div className="text-sm breadcrumbs">
        <ul>
          <li>
            <a href={`/${currentLang}`}>{t('home')}</a>
          </li>
          <li>
            <a href={`/${currentLang}/categories`}>{t('categories')}</a>
          </li>
          <li>{categoryName}</li>
        </ul>
      </div>

      {/* Page Header */}
      <div className="text-center space-y-4">
        <h1 className="text-5xl font-bold">{categoryName}</h1>
        <p className="text-xl opacity-70">
          {posts.length} {t('posts')}
        </p>
      </div>

      {loading && (
        <div className="flex justify-center">
          <span className="loading loading-spinner loading-lg"></span>
        </div>
      )}

      {error && (
        <div className="alert alert-error">
          <span>
            {t('error')}: {error.message}
          </span>
        </div>
      )}

      {!loading && !error && posts.length === 0 && (
        <div className="alert alert-info">
          <span>{t('noPosts')}</span>
        </div>
      )}

      {/* Posts Grid */}
      {!loading && !error && posts.length > 0 && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {posts.map((post: BlogPost) => {
            const thumbnailUrl = getThumbnailUrl(post);
            const translatedTitle = getTranslatedTitle(post);
            const translatedPreview = getTranslatedPreview(post);
            return (
              <div key={post.id} className="card bg-base-200 shadow-xl">
                {thumbnailUrl && (
                  <figure>
                    <img
                      src={thumbnailUrl}
                      alt={translatedTitle}
                      className="h-48 w-full object-cover"
                    />
                  </figure>
                )}
                <div className="card-body">
                  <h3 className="card-title">{translatedTitle}</h3>
                  <p className="text-sm opacity-70">
                    {formatDate(post.createdAt)}
                  </p>
                  <p>{translatedPreview.substring(0, 120)}...</p>
                  <div className="card-actions justify-end mt-4">
                    <a
                      href={`/${currentLang}/posts/${post.slug}`}
                      className="btn btn-primary btn-sm"
                    >
                      {t('readMore')}
                    </a>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
};

export default CategoryDetailPage;
