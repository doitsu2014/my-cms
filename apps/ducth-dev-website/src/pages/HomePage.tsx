import { useQuery } from '@apollo/client';
import { useParams, Link } from 'react-router-dom';
import { getAboutContent } from '../config/about.config';
import { getMediaBaseUrl } from '../config/api.config';
import ContentEmpty from '../components/feedback/ContentEmpty';
import ContentError from '../components/feedback/ContentError';
import ContentSkeleton from '../components/feedback/ContentSkeleton';
import Container from '../components/layout/Container';
import Section from '../components/layout/Section';
import SectionHeader from '../components/editorial/SectionHeader';
import PostCard from '../components/posts/PostCard';
import { GET_BLOG_POSTS } from '../infrastructure/graphql/queries';
import type { BlogPost } from '../types/content';

const HomePage = () => {
  const { lang = 'en' } = useParams<{ lang: string }>();
  const currentLang = lang === 'vi' ? 'vi' : 'en';
  const { loading, error, data, refetch } = useQuery(GET_BLOG_POSTS);
  const posts = ((data?.posts?.nodes || []) as BlogPost[])
    .filter((post) => post.published !== false)
    .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
  const categories = Array.from(
    posts.reduce((map, post) => {
      const category = post.categories;
      if (category?.slug) {
        const current = map.get(category.slug) || { category, count: 0 };
        current.count += 1;
        map.set(category.slug, current);
      }
      return map;
    }, new Map<string, { category: NonNullable<BlogPost['categories']>; count: number }>()),
  ).map(([, value]) => value);
  const mediaBaseUrl = getMediaBaseUrl();
  const about = getAboutContent(currentLang);

  if (loading) return <ContentSkeleton variant="home" />;
  if (error) return <ContentError lang={currentLang} onRetry={() => void refetch()} />;

  return (
    <div className="home-page">
      <Section className="home-recent">
        <SectionHeader
          eyebrow={currentLang === 'vi' ? 'Mục lục · Index' : 'Writing index'}
          title={currentLang === 'vi' ? 'Bài viết gần đây' : 'Recent articles'}
          description={currentLang === 'vi' ? 'Ghi chép về phần mềm, hệ thống và công việc phía sau chúng.' : 'Notes on software, systems, and the work around them.'}
        />
        <Container>
          {posts.length === 0 ? (
            <ContentEmpty lang={currentLang} message={currentLang === 'vi' ? 'Chưa có nội dung đã xuất bản.' : 'There is no published content here yet.'} href={`/${currentLang}/categories`} />
          ) : (
            <div className="home-post-grid">
              {posts.slice(0, 6).map((post, index) => (
                <PostCard
                  key={post.id}
                  post={post}
                  lang={currentLang}
                  mediaBaseUrl={mediaBaseUrl}
                  variant={index === 0 ? 'lead' : index === 1 ? 'compact' : index === 5 ? 'wide' : 'standard'}
                />
              ))}
            </div>
          )}
        </Container>
      </Section>

      {categories.length > 0 && (
        <Section tone="fresh-paper" className="category-strip">
          <Container>
            <div className="category-strip__header">
              <div>
                <p className="eyebrow"><span className="eyebrow__dot" aria-hidden="true" />{currentLang === 'vi' ? 'Mục lục' : 'Index'}</p>
                <h2 className="display-h2">{currentLang === 'vi' ? 'Danh mục' : 'Categories'}</h2>
              </div>
              <Link className="text-link" to={`/${currentLang}/categories`}>{currentLang === 'vi' ? 'Xem tất cả' : 'View all'} <span aria-hidden="true">→</span></Link>
            </div>
            <ul className="category-strip__list">
              {categories.map(({ category, count }) => (
                <li key={category.slug}>
                  <strong>{String(count).padStart(2, '0')}<span aria-hidden="true">.</span></strong>
                  <Link to={`/${currentLang}/categories/${category.slug}`}>{category.displayName}</Link>
                  <span>{category.slug}</span>
                </li>
              ))}
            </ul>
          </Container>
        </Section>
      )}

      {about.verified && about.contact.email && (
        <Section tone="ink" className="home-contact-cta">
          <Container>
            <p className="eyebrow">{about.contact.eyebrow}</p>
            <h2 className="display-h2">{about.contact.title}</h2>
            <p className="lead">{about.contact.body}</p>
            <a className="button" href={`mailto:${about.contact.email}`}>{currentLang === 'vi' ? 'Liên hệ' : 'Get in touch'} <span aria-hidden="true">→</span></a>
          </Container>
        </Section>
      )}
    </div>
  );
};

export default HomePage;
