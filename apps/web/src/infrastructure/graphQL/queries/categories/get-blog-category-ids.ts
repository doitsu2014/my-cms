import { gql } from "@apollo/client";

export default function buildGetBlogCategoryIdsQuery() {
  return gql`
    query GetBlogCategoryIds {
        categories(filters: { categoryType: { eq: Blog }, parentId: { is_null: "true" } }) {
          nodes {
            id
          }
        }
    }`;
}
