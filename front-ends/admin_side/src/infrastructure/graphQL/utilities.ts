import type { CategoryModel } from '@/domains/category';
import type { PostInFooterModel, PostModel } from '@/domains/post';

export const mapGraphQlModelToCategoryModel = (
  graphqlNode: any | undefined
): CategoryModel | undefined => {
  return !!graphqlNode
    ? {
        ...graphqlNode,
        translations: graphqlNode.translations?.nodes.map((node: any) => ({
          id: node.id,
          languageCode: node.languageCode,
          displayName: node.displayName,
          slug: node.slug
        })),
        tags: graphqlNode.tags?.nodes.map((node: any) => ({
          id: node.tags.id,
          name: node.tags.name,
          slug: node.tags.slug
        }))
      }
    : undefined;
};

export const mapGraphQlModelToPostModel = (graphqlNode: any | undefined): PostModel | undefined => {
  return !!graphqlNode
    ? {
        ...graphqlNode,
        categoryDisplayName: graphqlNode.categories?.displayName,
        categorySlug: graphqlNode.categories?.slug,
        translations: graphqlNode.translations?.nodes.map((node: any) => ({
          id: node.id,
          languageCode: node.languageCode,
          title: node.title,
          previewContent: node.previewContent,
          content: node.content,
          slug: node.slug
        })),
        tags: graphqlNode.tags?.nodes.map((node: any) => ({
          id: node.tags.id,
          name: node.tags.name,
          slug: node.tags.slug
        }))
      }
    : undefined;
};

export const mapGraphQlModelToPostInFooterModel = (
  graphqlNode: any | undefined
): PostInFooterModel | undefined => {
  return !!graphqlNode
    ? {
        ...graphqlNode,
        category: mapGraphQlModelToCategoryModel(graphqlNode.categories),
        translations: graphqlNode.translations?.nodes.map((node: any) => ({
          id: node.id,
          languageCode: node.languageCode,
          title: node.title,
          previewContent: node.previewContent,
          content: node.content,
          slug: node.slug
        }))
      }
    : undefined;
};
