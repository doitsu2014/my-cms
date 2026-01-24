import { z } from 'zod';

const postTranslationSchema = z.object({
  id: z.string().optional(),
  languageCode: z.string().min(2, 'Language code is required'),
  title: z.string().min(1, 'Title is required').max(200, 'Title must be less than 200 characters'),
  previewContent: z.string().min(1, 'Preview content is required'),
  content: z.string().min(1, 'Content is required'),
  slug: z.string().optional(),
});

export const blogFormSchema = z.object({
  title: z.string().min(1, 'Title is required').max(200, 'Title must be less than 200 characters'),
  previewContent: z.string().min(1, 'Preview content is required'),
  content: z.string().min(1, 'Content is required'),
  thumbnailPaths: z.array(z.string()),
  published: z.boolean(),
  tagNames: z.array(z.string()),
  categoryId: z.string().min(1, 'Category is required'),
  translations: z.array(postTranslationSchema),
  rowVersion: z.number(),
});

export type BlogFormData = z.infer<typeof blogFormSchema>;
export type PostTranslationData = z.infer<typeof postTranslationSchema>;
