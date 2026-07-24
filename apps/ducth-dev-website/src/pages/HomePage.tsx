import { useQuery } from '@apollo/client';
import { useTranslation } from 'react-i18next';
import { useParams } from 'react-router-dom';
import { GET_BLOG_POSTS } from '../infrastructure/graphql/queries';
import { SITE_CONFIG } from '../config/site.config';
import { getMediaUrl } from '../config/get-media-url';
import { readBrowserConfig } from '../config/read-browser-config';

interface BlogPost {
  id: string;
  title: string;
  slug: string;
  previewContent: string;
  content?: string;
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
      content?: string;
      previewContent?: string;
    }>;
  };
}

const HomePage = () => {
  const { t } = useTranslation();
  const { lang } = useParams<{ lang: string }>();
  const currentLang = lang || 'en';

  // Fetch blog posts
  const { loading, error, data } = useQuery(GET_BLOG_POSTS);

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

  // Get translated title
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

  // Filter and sort posts - published only, sorted by date (latest first)
  const allPosts = (data?.posts?.nodes?.filter((post: BlogPost) => post.published) || [])
    .sort((a: BlogPost, b: BlogPost) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
  
  const featuredPosts = allPosts.slice(0, 6); // Show top 6 featured posts

  return (
    <div className="space-y-8">
      {/* Hero Section */}
      <div className="hero min-h-[60vh] bg-base-200 rounded-lg">
        <div className="hero-content text-center">
          <div className="max-w-2xl">
            <div className="avatar mb-6">
              <div className="w-32 rounded-full ring ring-primary ring-offset-base-100 ring-offset-2">
                <img src={SITE_CONFIG.avatarUrl} alt="Duc Tran" />
              </div>
            </div>
            <h1 className="text-5xl font-bold">{t('welcome')}</h1>
            <p className="py-6 text-lg">{t('description')}</p>
            <button className="btn btn-primary">{t('exploreArticles')}</button>
          </div>
        </div>
      </div>

      {/* Featured Posts Section */}
      <section>
        <h2 className="text-3xl font-bold mb-6">{t('featuredPosts')}</h2>
        {loading && (
          <div className="flex justify-center">
            <span className="loading loading-spinner loading-lg"></span>
          </div>
        )}
        {error && (
          <div className="alert alert-error">
            <span>{t('error')}: {error.message}</span>
          </div>
        )}
        {!loading && !error && featuredPosts.length === 0 && (
          <div className="alert alert-info">
            <span>{t('noDataAvailable')}</span>
          </div>
        )}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {featuredPosts.map((post: BlogPost) => {
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
                  {post.categories && (
                    <div className="badge badge-primary">{post.categories.displayName}</div>
                  )}
                  <p>
                    {translatedPreview.substring(0, 100)}...
                  </p>
                  <div className="card-actions justify-end">
                    <a href={`/${currentLang}/posts/${post.slug}`} className="btn btn-primary btn-sm">
                      {t('readMore')}
                    </a>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </section>
    </div>
  );
};

export default HomePage;
