import { useQuery } from '@apollo/client';
import { useParams } from 'react-router-dom';
import { getLocalizedCategory } from '../lib/i18n/getLocalizedCategory';
import { getMediaBaseUrl } from '../config/api.config';
import Breadcrumbs from '../components/navigation/Breadcrumbs';
import ContentEmpty from '../components/feedback/ContentEmpty';
import ContentError from '../components/feedback/ContentError';
import ContentSkeleton from '../components/feedback/ContentSkeleton';
import NotFoundState from '../components/feedback/NotFoundState';
import Container from '../components/layout/Container';
import Section from '../components/layout/Section';
import PostListRow from '../components/posts/PostListRow';
import { GET_POSTS_BY_CATEGORY } from '../infrastructure/graphql/queries';
import type { BlogPost, Category } from '../types/content';

const CategoryDetailPage = () => {
  const { lang = 'en', slug = '' } = useParams<{ lang: string; slug: string }>();
  const currentLang = lang === 'vi' ? 'vi' : 'en';
  const { loading, error, data, refetch } = useQuery(GET_POSTS_BY_CATEGORY, { variables: { categorySlug: slug } });
  const category = (data?.categories?.nodes?.[0] || undefined) as Category | undefined;
  const posts = ((data?.posts?.nodes || []) as BlogPost[])
    .filter((post) => post.published !== false && post.categories?.slug === slug)
    .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
  const mediaBaseUrl = getMediaBaseUrl();

  if (loading) return <ContentSkeleton variant="category" />;
  if (error) return <ContentError lang={currentLang} onRetry={() => void refetch()} />;
  if (!category) return <NotFoundState lang={currentLang} message={currentLang === 'vi' ? 'Không tìm thấy danh mục này.' : 'This category could not be found.'} />;

  const localizedCategory = getLocalizedCategory(category, currentLang);
  return (
    <div className="category-detail-page">
      <Breadcrumbs items={[
        { label: currentLang === 'vi' ? 'Trang chủ' : 'Home', href: `/${currentLang}` },
        { label: currentLang === 'vi' ? 'Danh mục' : 'Categories', href: `/${currentLang}/categories` },
        { label: localizedCategory.displayName },
      ]} />
      <Section className="category-detail-intro">
        <Container>
          <div className="category-detail-intro__grid">
            <img className="category-detail-intro__image" src="/images/architecture.jpg" alt={localizedCategory.displayName} />
            <div>
              <p className="eyebrow"><span className="eyebrow__dot" aria-hidden="true" />{currentLang === 'vi' ? 'Danh mục' : 'Category'}</p>
              <h1 className="display-h1">{localizedCategory.displayName}</h1>
              <p className="page-count">{posts.length} {currentLang === 'vi' ? 'bài viết đã xuất bản' : 'published articles'}</p>
            </div>
          </div>
        </Container>
      </Section>
      <Section className="category-posts">
        <Container>
          {posts.length === 0 ? (
            <ContentEmpty lang={currentLang} message={currentLang === 'vi' ? 'Danh mục này chưa có bài viết đã xuất bản.' : 'No published posts in this category yet.'} href={`/${currentLang}/categories`} />
          ) : (
            <>
              <p className="showing-all">{currentLang === 'vi' ? `Hiển thị tất cả ${posts.length} bài viết` : `Showing all ${posts.length} articles`}</p>
              {posts.map((post) => <PostListRow key={post.id} post={post} lang={currentLang} mediaBaseUrl={mediaBaseUrl} />)}
            </>
          )}
        </Container>
      </Section>
    </div>
  );
};

export default CategoryDetailPage;
