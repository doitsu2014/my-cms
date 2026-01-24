import { CategoryTypeEnum } from '@/domains/category';

export interface UpdateCategoryModel {
  id: string;
  displayName: string;
  categoryType: CategoryTypeEnum;
  tagNames: string[];
  rowVersion: number;
  translations: {id?: string, languageCode: string, displayName: string}[];
}
