import { CategoryTypeEnum } from '@/domains/category';

export interface CreateCategoryModel {
  displayName: string;
  categoryType: CategoryTypeEnum;
  tagNames: string[];
  translations: {languageCode: string, displayName: string}[];
}
