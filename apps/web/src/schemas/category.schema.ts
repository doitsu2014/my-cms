import { z } from 'zod';
import { CategoryTypeEnum } from '@/domains/category';

const translationSchema = z.object({
  id: z.string().optional(),
  languageCode: z.string().min(2, 'Language code is required'),
  displayName: z.string().min(1, 'Display name is required'),
});

export const categoryFormSchema = z.object({
  displayName: z.string().min(1, 'Display name is required').max(100, 'Display name must be less than 100 characters'),
  categoryType: z.enum(CategoryTypeEnum),
  tagNames: z.array(z.string()),
  translations: z.array(translationSchema),
  rowVersion: z.number(),
});

export type CategoryFormData = z.infer<typeof categoryFormSchema>;
export type TranslationData = z.infer<typeof translationSchema>;
