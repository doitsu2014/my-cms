import { useQuery } from '@apollo/client';
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css';
import { useEffect, useRef } from 'react';
import { useParams } from 'react-router-dom';
import { SITE_CONFIG } from '../config/site.config';
import { getMediaBaseUrl } from '../config/api.config';
import { getLocalizedPost } from '../lib/i18n/getLocalizedPost';
import { getLocalizedCategory } from '../lib/i18n/getLocalizedCategory';
import { getPostThumbnail } from '../lib/media/getPostThumbnail';
import ArticleProse from '../components/posts/ArticleProse';
import Breadcrumbs from '../components/navigation/Breadcrumbs';
import ContentError from '../components/feedback/ContentError';
import ContentSkeleton from '../components/feedback/ContentSkeleton';
import NotFoundState from '../components/feedback/NotFoundState';
import Container from '../components/layout/Container';
import Section from '../components/layout/Section';
import CategoryLabel from '../components/editorial/CategoryLabel';
import PostMeta from '../components/editorial/PostMeta';
import PostArtwork from '../components/posts/PostArtwork';
import RelatedPosts from '../components/posts/RelatedPosts';
import ShareActions from '../components/posts/ShareActions';
import { GET_BLOG_POST_BY_SLUG, GET_BLOG_POSTS } from '../infrastructure/graphql/queries';
import type { BlogPost } from '../types/content';

const PostDetailPage = () => {
  const { lang = 'en', slug = '' } = useParams<{ lang: string; slug: string }>();
  const currentLang = lang === 'vi' ? 'vi' : 'en';
  const contentRef = useRef<HTMLDivElement>(null);
  const postQuery = useQuery(GET_BLOG_POST_BY_SLUG, { variables: { slug } });
  const allPostsQuery = useQuery(GET_BLOG_POSTS);
  const post = postQuery.data?.posts?.nodes?.[0] as BlogPost | undefined;
  const allPosts = (allPostsQuery.data?.posts?.nodes || []) as BlogPost[];
  const mediaBaseUrl = getMediaBaseUrl();

  useEffect(() => {
    if (!contentRef.current || !post) return;
    contentRef.current.querySelectorAll('pre code').forEach((block) => {
      if (!block.classList.contains('hljs')) hljs.highlightElement(block as HTMLElement);
    });
  }, [post, currentLang]);

  if (postQuery.loading || allPostsQuery.loading) return <ContentSkeleton variant="post" />;
  if (postQuery.error || allPostsQuery.error) return <ContentError lang={currentLang} onRetry={() => { void postQuery.refetch(); void allPostsQuery.refetch(); }} />;
  if (!post) return <NotFoundState lang={currentLang} message={currentLang === 'vi' ? 'Không tìm thấy bài viết này.' : 'This post could not be found.'} />;

  const localizedPost = getLocalizedPost(post, currentLang);
  const localizedCategory = post.categories ? getLocalizedCategory(post.categories, currentLang) : undefined;
  const thumbnail = getPostThumbnail(post, mediaBaseUrl);
  const siteUrl = SITE_CONFIG.siteUrl.replace(/\/$/, '');
  const canonicalUrl = `${siteUrl}/${currentLang}/posts/${post.slug}`;

  return (
    <article className="post-detail-page">
      <Breadcrumbs items={[
        { label: currentLang === 'vi' ? 'Trang chủ' : 'Home', href: `/${currentLang}` },
        { label: currentLang === 'vi' ? 'Danh mục' : 'Categories', href: `/${currentLang}/categories` },
        ...(localizedCategory ? [{ label: localizedCategory.displayName, href: `/${currentLang}/categories/${localizedCategory.slug}` }] : []),
        { label: localizedPost.title },
      ]} />
      <Section className="post-header">
        <Container>
          <div className="post-header__inner">
            {localizedCategory && <CategoryLabel>{localizedCategory.displayName}</CategoryLabel>}
            <PostMeta date={post.createdAt} lang={currentLang} />
            <h1 className="display-h1">{localizedPost.title}</h1>
            {localizedPost.previewContent && <p className="post-deck">{localizedPost.previewContent}</p>}
            {post.createdBy && <p className="post-byline">{currentLang === 'vi' ? 'bởi' : 'by'} <strong>{post.createdBy}</strong></p>}
          </div>
        </Container>
      </Section>
      <Container>
        <PostArtwork src={thumbnail} slug={post.slug} title={localizedPost.title} aspect="21 / 9" />
        <div ref={contentRef}>
          <ArticleProse html={localizedPost.content} />
        </div>
        <ShareActions canonicalUrl={canonicalUrl} title={localizedPost.title} lang={currentLang} />
      </Container>
      <RelatedPosts currentPost={post} posts={allPosts} lang={currentLang} mediaBaseUrl={mediaBaseUrl} />
    </article>
  );
};

export default PostDetailPage;
