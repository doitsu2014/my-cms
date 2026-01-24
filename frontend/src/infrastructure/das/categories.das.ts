import { buildGraphQLClient } from "../graphQL/graphql-client";
import buildGetBlogCategoryIdsQuery from "../graphQL/queries/categories/get-blog-category-ids";

export const getBlogCategoryIds = async (): Promise<string[]> => {
    try {
        const { data } = await buildGraphQLClient().query({
            query: buildGetBlogCategoryIdsQuery(),
            fetchPolicy: 'no-cache'
        });
        return (data?.categories?.nodes || []).map((category: { id: string }) => category.id);
    } catch (error) {
        console.error('Error fetching blog category ids:', error);
        return [];
    }
};
