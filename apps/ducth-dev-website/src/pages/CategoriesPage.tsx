import { useQuery } from '@apollo/client';
import { Link, useParams } from 'react-router-dom';
import { getLocalizedCategory } from '../lib/i18n/getLocalizedCategory';
import { getMediaBaseUrl } from '../config/api.config';
import ContentEmpty from '../components/feedback/ContentEmpty';
import ContentError from '../components/feedback/ContentError';
import ContentSkeleton from '../components/feedback/ContentSkeleton';
import Container from '../components/layout/Container';
import Section from '../components/layout/Section';
import PostCard from '../components/posts/PostCard';
import { GET_BLOG_POSTS, GET_CATEGORIES } from '../infrastructure/graphql/queries';
import type { BlogPost, Category } from '../types/content';

const CategoriesPage = () => {
  const { lang = 'en' } = useParams<{ lang: string }>();
  const currentLang = lang === 'vi' ? 'vi' : 'en';
  const categoriesQuery = useQuery(GET_CATEGORIES);
  const postsQuery = useQuery(GET_BLOG_POSTS);
  const loading = categoriesQuery.loading || postsQuery.loading;
  const error = categoriesQuery.error || postsQuery.error;
  const categories = ((categoriesQuery.data?.categories?.nodes || []) as Category[]).filter(
    (category) => !category.categoryType || category.categoryType === 'Blog',
  );
  const posts = ((postsQuery.data?.posts?.nodes || []) as BlogPost[])
    .filter((post) => post.published !== false)
    .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
  const mediaBaseUrl = getMediaBaseUrl();
  const totalPosts = posts.length;

  if (loading) return <ContentSkeleton variant="categories" />;
  if (error) return <ContentError lang={currentLang} onRetry={() => { void categoriesQuery.refetch(); void postsQuery.refetch(); }} />;

  return (
    <div className="categories-page">
      <Section className="categories-header">
        <Container>
          <p className="eyebrow"><span className="eyebrow__dot" aria-hidden="true" />{currentLang === 'vi' ? 'Mục lục · Index' : 'Index'}</p>
          <h1 className="display-h1">{currentLang === 'vi' ? 'Danh mục' : 'Categories'}</h1>
          <p className="lead">{currentLang === 'vi' ? 'Mục lục yên tĩnh của những chủ đề trong notebook này.' : 'A quiet index of the subjects covered by this notebook.'}</p>
          <p className="page-count">{categories.length} {currentLang === 'vi' ? 'danh mục' : 'categories'} · {totalPosts} {currentLang === 'vi' ? 'bài viết đã xuất bản' : 'published posts'}</p>
        </Container>
      </Section>

      <Section className="category-list">
        <Container>
          {categories.length === 0 ? (
            <ContentEmpty lang={currentLang} message={currentLang === 'vi' ? 'Chưa có danh mục.' : 'There are no categories yet.'} />
          ) : (
            categories.map((category, index) => {
              const localizedCategory = getLocalizedCategory(category, currentLang);
              const categoryPosts = posts.filter((post) => post.categories?.slug === category.slug).slice(0, 3);
              return (
                <article className="category-row" key={category.id || category.slug}>
                  <div className="category-row__ordinal">{String(index + 1).padStart(2, '0')}<span aria-hidden="true">.</span></div>
                  <div className="category-row__intro">
                    <Link className="category-row__name" to={`/${currentLang}/categories/${category.slug}`}>
                      {localizedCategory.displayName}
                    </Link>
                    <span className="category-row__slug">/{localizedCategory.slug}</span>
                    <Link className="text-link" to={`/${currentLang}/categories/${category.slug}`}>
                      {currentLang === 'vi' ? 'Vào danh mục' : 'View category'} <span aria-hidden="true">→</span>
                    </Link>
                  </div>
                  <div className="category-row__posts">
                    <p className="category-row__count">{categoryPosts.length} {currentLang === 'vi' ? 'bài mới nhất' : 'latest posts'}</p>
                    {categoryPosts.length === 0 ? (
                      <p className="muted-copy">{currentLang === 'vi' ? 'Chưa có bài viết.' : 'No published posts yet.'}</p>
                    ) : (
                      <div className="category-row__preview-grid">
                        {categoryPosts.map((post) => <PostCard key={post.id} post={post} lang={currentLang} variant="compact" mediaBaseUrl={mediaBaseUrl} />)}
                      </div>
                    )}
                  </div>
                </article>
              );
            })
          )}
        </Container>
      </Section>
    </div>
  );
};

export default CategoriesPage;
