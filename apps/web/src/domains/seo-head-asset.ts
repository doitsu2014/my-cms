import { z } from 'zod';
import { authenticatedFetch, getApiUrl } from '@/config/api.config';

export const seoHeadAssetFormSchema = z.object({
  label: z.string().trim().min(1, 'Label is required').max(128, 'Label is too long'),
  html: z.string().min(1, 'Source HTML is required').max(32768, 'Source HTML is too large'),
  enabled: z.boolean(),
  sortOrder: z.coerce.number().int().positive('Sort order must be positive'),
});

export type SeoHeadAssetFormValues = z.infer<typeof seoHeadAssetFormSchema>;

export interface SeoHeadAsset extends SeoHeadAssetFormValues {
  id: string;
  rowVersion: number;
  createdAt: string;
  updatedAt: string;
  createdBy: string;
  updatedBy: string;
}

export interface SeoHeadAssetResponse extends SeoHeadAssetFormValues {
  id: string;
  updatedAt: string;
}

interface ApiEnvelope<T> { data: T; message?: string }

export class SeoHeadAssetApiError extends Error {
  constructor(public readonly status: number, public readonly code?: string, message = 'SEO head asset request failed') {
    super(message);
    this.name = 'SeoHeadAssetApiError';
  }
}

async function readResponse<T>(response: Response): Promise<T> {
  let payload: ApiEnvelope<T> & { errorCode?: string; errors?: string[] };
  try { payload = await response.json(); } catch { throw new SeoHeadAssetApiError(response.status); }
  if (!response.ok) throw new SeoHeadAssetApiError(response.status, payload.errorCode, payload.errors?.join(', '));
  return payload.data;
}

export async function listSeoHeadAssets(token: string | null): Promise<SeoHeadAsset[]> {
  return readResponse(await authenticatedFetch(getApiUrl('/seo/head-assets'), token, { cache: 'no-store' }));
}
export async function getSeoHeadAsset(token: string | null, id: string): Promise<SeoHeadAsset> {
  return readResponse(await authenticatedFetch(getApiUrl(`/seo/head-assets/${encodeURIComponent(id)}`), token, { cache: 'no-store' }));
}
export async function createSeoHeadAsset(token: string | null, values: SeoHeadAssetFormValues): Promise<SeoHeadAsset> {
  return readResponse(await authenticatedFetch(getApiUrl('/seo/head-assets'), token, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(values) }));
}
export async function updateSeoHeadAsset(token: string | null, id: string, values: SeoHeadAssetFormValues, rowVersion: number): Promise<SeoHeadAsset> {
  return readResponse(await authenticatedFetch(getApiUrl(`/seo/head-assets/${encodeURIComponent(id)}`), token, { method: 'PUT', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ ...values, rowVersion }) }));
}
export async function deleteSeoHeadAsset(token: string | null, id: string): Promise<void> {
  const response = await authenticatedFetch(getApiUrl(`/seo/head-assets/${encodeURIComponent(id)}`), token, { method: 'DELETE' });
  if (!response.ok) await readResponse(response);
}
