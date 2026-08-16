import { useQuery } from '@apollo/client';
import { useLocation } from 'react-router-dom';
import { SITE_CONFIG } from '../config/site.config';
import { getRuntimeConfig } from '../config/get-runtime-config';
import {
  GET_BLOG_POST_BY_SLUG,
  GET_POSTS_BY_CATEGORY,
} from '../infrastructure/graphql/queries';
import { replacePublicHead } from './public-head';
import {
  resolvePublicMetadata,
  type PublicMetadataState,
  type PublicSiteConfig,
} from './public-metadata';
import type { BlogPost, Category } from '../types/content';
import { useEffect } from 'react';

function metadataConfig(): PublicSiteConfig | null {
  try {
    return {
      siteName: SITE_CONFIG.siteName,
      siteUrl: SITE_CONFIG.siteUrl,
      defaultTitle: SITE_CONFIG.seo.defaultTitle,
      defaultDescription: SITE_CONFIG.seo.defaultDescription,
      avatarUrl: SITE_CONFIG.avatarUrl,
      mediaBaseUrl: getRuntimeConfig().mediaBaseUrl,
    };
  } catch {
    return null;
  }
}

function StaticPublicMetadataBoundary({
  location,
}: {
  location: ReturnType<typeof useLocation>;
}) {
  useEffect(() => {
    const config = metadataConfig();
    if (!config) return;
    replacePublicHead(
      resolvePublicMetadata({
        url: `${location.pathname}${location.search}${location.hash}`,
        config,
      }),
    );
  }, [location.hash, location.pathname, location.search]);
  return null;
}

function DynamicPublicMetadataBoundary({
  location,
  postMatch,
  categoryMatch,
}: {
  location: ReturnType<typeof useLocation>;
  postMatch: RegExpMatchArray | null;
  categoryMatch: RegExpMatchArray | null;
}) {
  const postQuery = useQuery(GET_BLOG_POST_BY_SLUG, {
    variables: { slug: postMatch?.[1] || '' },
    skip: !postMatch,
  });
  const categoryQuery = useQuery(GET_POSTS_BY_CATEGORY, {
    variables: { categorySlug: categoryMatch?.[1] || '' },
    skip: !categoryMatch,
  });

  const post = postMatch
    ? (postQuery.data?.posts?.nodes?.[0] as BlogPost | undefined)
    : undefined;
  const category = categoryMatch
    ? (categoryQuery.data?.categories?.nodes?.[0] as Category | undefined)
    : undefined;
  const state: PublicMetadataState = postMatch
    ? postQuery.loading
      ? 'loading'
      : postQuery.error
        ? 'error'
        : post
          ? 'ready'
          : 'not-found'
    : categoryMatch
      ? categoryQuery.loading
        ? 'loading'
        : categoryQuery.error
          ? 'error'
          : category
            ? 'ready'
            : 'not-found'
      : 'ready';

  useEffect(() => {
    const config = metadataConfig();
    if (!config) return;
    replacePublicHead(
      resolvePublicMetadata({
        url: `${location.pathname}${location.search}${location.hash}`,
        config,
        post,
        category,
        state,
      }),
    );
  }, [
    category,
    location.hash,
    location.pathname,
    location.search,
    post,
    state,
  ]);

  return null;
}

export default function PublicMetadataBoundary() {
  const location = useLocation();
  const postMatch = location.pathname.match(/^\/(?:en|vi)\/posts\/([^/]+)/);
  const categoryMatch = location.pathname.match(
    /^\/(?:en|vi)\/categories\/([^/]+)/,
  );
  if (!postMatch && !categoryMatch)
    return <StaticPublicMetadataBoundary location={location} />;
  return (
    <DynamicPublicMetadataBoundary
      location={location}
      postMatch={postMatch}
      categoryMatch={categoryMatch}
    />
  );
}
