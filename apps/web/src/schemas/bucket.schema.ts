import { z } from 'zod';

/**
 * Bucket name pattern: must start with a lowercase letter,
 * followed by 2-62 lowercase letters, digits, dashes, or underscores.
 * Total length: 3-63 characters.
 */
export const BUCKET_NAME_PATTERN = /^[a-z][a-z0-9_-]{2,62}$/;
export const BUCKET_NAME_MESSAGE =
  'must start with a lowercase letter; only [a-z0-9_-] allowed';

export const bucketNameSchema = z
  .string()
  .regex(BUCKET_NAME_PATTERN, BUCKET_NAME_MESSAGE);

const allowedMimeTypesSchema = z
  .string()
  .max(2000, 'Allowed MIME types too long')
  .optional();

const fileSizeLimitSchema = z
  .string()
  .optional()
  .refine(
    (v) => v === undefined || v === '' || (Number.isFinite(Number(v)) && Number(v) > 0),
    'File size limit must be a positive number',
  );

export const createBucketFromTextFormSchema = z.object({
  name: bucketNameSchema,
  public: z.boolean(),
  fileSizeLimit: fileSizeLimitSchema,
  allowedMimeTypes: allowedMimeTypesSchema,
});

export type CreateBucketTextFormData = z.infer<typeof createBucketFromTextFormSchema>;

export const updateBucketFromTextFormSchema = z
  .object({
    public: z.boolean(),
    fileSizeLimit: fileSizeLimitSchema,
    allowedMimeTypes: allowedMimeTypesSchema,
  })
  .refine(
    (data) =>
      data.public !== false ||
      (data.fileSizeLimit !== undefined && data.fileSizeLimit !== '') ||
      (data.allowedMimeTypes !== undefined && data.allowedMimeTypes !== ''),
    { message: 'at least one field must be present' },
  );

export type UpdateBucketTextFormData = z.infer<typeof updateBucketFromTextFormSchema>;

export const createBucketSchema = z.object({
  name: bucketNameSchema,
  public: z.boolean().default(false),
  fileSizeLimit: z
    .number()
    .int('File size limit must be an integer')
    .positive('File size limit must be positive')
    .optional(),
  allowedMimeTypes: z.array(z.string().min(1)).max(100, 'Too many MIME types').optional(),
});

export type CreateBucketFormData = z.infer<typeof createBucketSchema>;

export const updateBucketSchema = z
  .object({
    public: z.boolean().optional(),
    fileSizeLimit: z
      .number()
      .int()
      .positive()
      .nullable()
      .optional(),
    allowedMimeTypes: z.array(z.string().min(1)).max(100).nullable().optional(),
  })
  .refine(
    (data) =>
      data.public !== undefined ||
      data.fileSizeLimit !== undefined ||
      data.allowedMimeTypes !== undefined,
    { message: 'at least one field must be present' },
  );

export type UpdateBucketFormData = z.infer<typeof updateBucketSchema>;

/**
 * Convert a comma-separated MIME types string into an array.
 * Returns undefined for empty / whitespace input.
 */
export function parseMimeTypesList(input: string | undefined): string[] | undefined {
  if (!input) return undefined;
  const items = input
    .split(',')
    .map((s) => s.trim())
    .filter((s) => s.length > 0);
  return items.length > 0 ? items : undefined;
}

/**
 * Convert a file size limit string (text input) to a number, or undefined.
 */
export function parseFileSizeLimit(input: string | undefined): number | undefined {
  if (input === undefined || input === '') return undefined;
  const n = Number(input);
  return Number.isFinite(n) && n > 0 ? n : undefined;
}
