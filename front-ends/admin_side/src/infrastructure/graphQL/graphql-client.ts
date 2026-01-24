import { ApolloClient, InMemoryCache, createHttpLink } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';
import { getAuthHeaders } from '../utilities.auth';
import { config } from '../../config/runtime-config';

// HTTP link to my-cms GraphQL API
const httpLink = createHttpLink({
  uri: config().graphqlApiUrl || 'http://localhost:4000/graphql',
});

// Auth link to add Keycloak Bearer token to requests
const authLink = setContext((_, { headers }) => {
  const authHeaders = getAuthHeaders();
  return {
    headers: {
      ...headers,
      ...authHeaders,
    }
  };
});

/**
 * Build Apollo GraphQL Client for my-cms backend
 * Configured with Keycloak authentication
 * Backend: https://github.com/doitsu2014/my-cms
 */
export const buildGraphQLClient = () =>
  new ApolloClient({
    link: authLink.concat(httpLink),
    cache: new InMemoryCache(),
    defaultOptions: {
      watchQuery: {
        fetchPolicy: 'cache-and-network',
      },
      query: {
        fetchPolicy: 'network-only',
      },
    },
  });

/**
 * Build GraphQL Client for cache API (optional)
 */
export const buildCacheGraphQLClient = () =>
  new ApolloClient({
    link: authLink.concat(createHttpLink({
      uri: config().graphqlCacheApiUrl || config().graphqlApiUrl || 'http://localhost:4000/graphql',
    })),
    cache: new InMemoryCache(),
  });
