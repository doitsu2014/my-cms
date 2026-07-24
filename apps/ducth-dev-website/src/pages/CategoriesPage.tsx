import { useQuery } from '@apollo/client';
import { useTranslation } from 'react-i18next';
import { useParams } from 'react-router-dom';
import { GET_CATEGORIES, GET_BLOG_POSTS } from '../infrastructure/graphql/queries';

interface Category {
  id: string;
  displayName: string;
  slug: string;
  categoryType: string;
  categoryTranslations?: {
    nodes: Array<{
      id: string;
      languageCode: string;
      displayName: string;
      slug: string;
    }>;
  };
  categoryTags?: {
    nodes: Array<{
      tags: {
        id: string;
        name: string;
        slug: string;
      };
    }>;
  };
}

const CategoriesPage = () => {
  const { t } = useTranslation();
  const { lang } = useParams<{ lang: string }>();
  const currentLang = lang || 'en';

  // Fetch categories
  const { loading, error, data } = useQuery(GET_CATEGORIES);

  // Helper to get translated name
  const getTranslatedName = (category: Category) => {
    if (currentLang !== 'en' && category.categoryTranslations?.nodes) {
      const translation = category.categoryTranslations.nodes.find(
        (t) => t.languageCode === currentLang
      );
      if (translation?.displayName) return translation.displayName;
    }
    return category.displayName;
  };

  // Fetch all posts to count posts per category
  const { data: postsData } = useQuery(GET_BLOG_POSTS);
  
  // Get blog categories only
  const categories = (data?.categories?.nodes || []).filter(
    (cat: Category) => cat.categoryType === 'Blog'
  );

  // Count posts per category
  const getPostCount = (categorySlug: string) => {
    if (!postsData?.posts?.nodes) return 0;
    return postsData.posts.nodes.filter(
      (post: { published: boolean; categories?: { slug: string } }) => post.published && post.categories?.slug === categorySlug
    ).length;
  };

  // Color palette for categories
  const colors = ['primary', 'secondary', 'accent', 'info', 'success', 'warning'];

  return (
    <div className="space-y-8">
      {/* Page Header */}
      <div className="text-center space-y-4">
        <h1 className="text-5xl font-bold">{t('browseCategories')}</h1>
        <p className="text-xl opacity-70">{t('categoriesDescription')}</p>
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

      {!loading && !error && categories.length === 0 && (
        <div className="alert alert-info">
          <span>{t('noDataAvailable')}</span>
        </div>
      )}

      {/* Categories Grid */}
      {!loading && !error && categories.length > 0 && (
        <>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-8">
            {categories.map((category: Category, index: number) => {
              const postCount = getPostCount(category.slug);
              const colorClass = colors[index % colors.length];

              return (
                <div
                  key={category.id}
                  className="card bg-base-200 shadow-xl hover:shadow-2xl transition-shadow"
                >
                  <div className="card-body">
                    <div className={`badge badge-${colorClass} mb-2`}>
                      {postCount} {t('posts')}
                    </div>
                    <h2 className="card-title">{getTranslatedName(category)}</h2>
                    <p className="opacity-80">
                      {category.slug.replace(/-/g, ' ')}
                    </p>
                    <div className="card-actions justify-end mt-4">
                      <a
                        href={`/${currentLang}/categories/${category.slug}`}
                        className="btn btn-primary btn-sm"
                      >
                        {t('viewPosts')}
                      </a>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>

          {/* Category Stats */}
          <div className="stats stats-vertical lg:stats-horizontal shadow w-full mt-12">
            <div className="stat">
              <div className="stat-title">{t('totalCategories')}</div>
              <div className="stat-value">{categories.length}</div>
              <div className="stat-desc">{t('organizedTopics')}</div>
            </div>

            <div className="stat">
              <div className="stat-title">{t('totalPosts')}</div>
              <div className="stat-value">
                {categories.reduce(
                  (sum: number, cat: Category) =>
                    sum + getPostCount(cat.slug),
                  0
                )}
              </div>
              <div className="stat-desc">{t('acrossAllCategories')}</div>
            </div>

            <div className="stat">
              <div className="stat-title">{t('mostPopular')}</div>
              <div className="stat-value text-primary">
                {categories.length > 0
                  ? getTranslatedName(
                      categories.reduce((max: Category, cat: Category) =>
                        getPostCount(cat.slug) > getPostCount(max.slug)
                          ? cat
                          : max
                      )
                    ).split(' ')[0]
                  : '-'}
              </div>
              <div className="stat-desc">{t('categoryWithMostPosts')}</div>
            </div>
          </div>
        </>
      )}
    </div>
  );
};

export default CategoriesPage;
