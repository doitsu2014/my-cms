import { useQuery } from '@apollo/client';
import { useTranslation } from 'react-i18next';
import { useParams } from 'react-router-dom';
import { useEffect, useRef } from 'react';
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css';
import { GET_BLOG_POST_BY_SLUG, GET_BLOG_POSTS } from '../infrastructure/graphql/queries';
import { getMediaUrl } from '../config/get-media-url';
import { readBrowserConfig } from '../config/read-browser-config';

interface BlogPost {
  id: string;
  title: string;
  slug: string;
  previewContent: string;
  content: string;
  createdAt: string;
  createdBy: string;
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
      content?: string;
    }>;
  };
}

const PostDetailPage = () => {
  const { t } = useTranslation();
  const { lang, slug } = useParams<{ lang: string; slug: string }>();
  const currentLang = lang || 'en';
  const contentRef = useRef<HTMLDivElement>(null);

  const { loading, error, data } = useQuery(GET_BLOG_POST_BY_SLUG, {
    variables: { slug },
  });

  // Fetch all posts for related posts
  const { data: allPostsData } = useQuery(GET_BLOG_POSTS);

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

  // Get translated content
  const getTranslatedContent = (post: BlogPost) => {
    if (currentLang !== 'en' && post.postTranslations?.nodes) {
      const translation = post.postTranslations.nodes.find(
        (t) => t.languageCode === currentLang
      );
      if (translation?.content) return translation.content;
    }
    return post.content;
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

  const post = data?.posts?.nodes?.[0];
  const thumbnailUrl = post ? getThumbnailUrl(post) : undefined;
  const translatedTitle = post ? getTranslatedTitle(post) : '';
  const translatedContent = post ? getTranslatedContent(post) : '';

  // Get related posts from same category (excluding current post)
  const relatedPosts = post
    ? (allPostsData?.posts?.nodes || [])
        .filter(
          (p: BlogPost) =>
            p.published &&
            p.id !== post.id &&
            p.categories?.slug === post.categories?.slug
        )
        .slice(0, 3)
    : [];

  // Apply syntax highlighting to code blocks after content is rendered
  useEffect(() => {
    if (contentRef.current && !loading && post) {
      const codeBlocks = contentRef.current.querySelectorAll('pre code');
      codeBlocks.forEach((block) => {
        hljs.highlightElement(block as HTMLElement);
      });
    }
  }, [loading, post, translatedContent]);

  return (
    <div className="space-y-8">
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

      {!loading && !error && !post && (
        <div className="alert alert-info">
          <span>{t('postNotFound')}</span>
        </div>
      )}

      {!loading && !error && post && (
        <>
          {/* Breadcrumb */}
          <div className="text-sm breadcrumbs">
            <ul>
              <li>
                <a href={`/${currentLang}`}>{t('home')}</a>
              </li>
              <li>
                <a href={`/${currentLang}/categories`}>{t('categories')}</a>
              </li>
              {post.categories && (
                <li>
                  <a href={`/${currentLang}/categories/${post.categories.slug}`}>
                    {post.categories.displayName}
                  </a>
                </li>
              )}
              <li>{translatedTitle}</li>
            </ul>
          </div>

          {/* Post Header */}
          <div className="space-y-4">
            <h1 className="text-5xl font-bold">{translatedTitle}</h1>
            <div className="flex flex-wrap gap-4 items-center text-sm opacity-70">
              <span>
                {t('postedOn')} {formatDate(post.createdAt)}
              </span>
              {post.categories && (
                <>
                  <span>•</span>
                  <a
                    href={`/${currentLang}/categories/${post.categories.slug}`}
                    className="badge badge-primary"
                  >
                    {post.categories.displayName}
                  </a>
                </>
              )}
              {post.createdBy && (
                <>
                  <span>•</span>
                  <span>{t('by')} {post.createdBy}</span>
                </>
              )}
            </div>
          </div>

          {/* Featured Image */}
          {thumbnailUrl && (
            <figure className="rounded-lg overflow-hidden">
              <img
                src={thumbnailUrl}
                alt={translatedTitle}
                className="w-full h-96 object-cover"
              />
            </figure>
          )}

          {/* Post Content - HTML from Tiptap */}
          <article className="prose prose-lg max-w-none">
            <div ref={contentRef} dangerouslySetInnerHTML={{ __html: translatedContent }} />
          </article>

          {/* Share Buttons */}
          <div className="divider">{t('share')}</div>
          <div className="flex gap-4 justify-center">
            <button className="btn btn-circle btn-outline">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-6 w-6"
                fill="currentColor"
                viewBox="0 0 24 24"
              >
                <path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z" />
              </svg>
            </button>
            <button className="btn btn-circle btn-outline">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-6 w-6"
                fill="currentColor"
                viewBox="0 0 24 24"
              >
                <path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z" />
              </svg>
            </button>
            <button className="btn btn-circle btn-outline">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-6 w-6"
                fill="currentColor"
                viewBox="0 0 24 24"
              >
                <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" />
              </svg>
            </button>
          </div>

          {/* Related Posts */}
          {relatedPosts.length > 0 && (
            <section className="mt-12">
              <h2 className="text-3xl font-bold mb-6">{t('relatedPosts')}</h2>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                {relatedPosts.map((relatedPost: BlogPost) => {
                  const relatedThumbnail = getThumbnailUrl(relatedPost);
                  const relatedTitle = getTranslatedTitle(relatedPost);
                  const relatedPreview = getTranslatedPreview(relatedPost);
                  return (
                    <div
                      key={relatedPost.id}
                      className="card bg-base-200 shadow-xl"
                    >
                      {relatedThumbnail && (
                        <figure>
                          <img
                            src={relatedThumbnail}
                            alt={relatedTitle}
                            className="h-48 w-full object-cover"
                          />
                        </figure>
                      )}
                      <div className="card-body">
                        <h3 className="card-title text-lg">{relatedTitle}</h3>
                        <p className="text-sm">
                          {relatedPreview.substring(0, 80)}...
                        </p>
                        <div className="card-actions justify-end mt-2">
                          <a
                            href={`/${currentLang}/posts/${relatedPost.slug}`}
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
            </section>
          )}
        </>
      )}
    </div>
  );
};

export default PostDetailPage;
