import { gql } from '@apollo/client';

export default function buildGetCategoriesQuery() {
  return gql`
    query GetCategories {
      categories {
        nodes {
          id
          displayName
          slug
          categoryType
          createdBy
          createdAt
          rowVersion
          tags {
            nodes {
              tags {
                id
                name
                slug
              }
            }
          }
        }
      }
    }
  `;
}

export function buildGetCategoriesWithTranslationsQuery() {
  return gql`
    query GetCategories {
      categories {
        nodes {
          id
          displayName
          slug
          categoryType
          createdBy
          createdAt
          rowVersion
          tags {
            nodes {
              tags {
                id
                name
                slug
              }
            }
          }
          translations {
            nodes {
              id
              languageCode
              displayName
            }
          }
        }
      }
    }
  `;
}
