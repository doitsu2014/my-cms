import { describe, expect, it, vi } from 'vitest';
import { createHeadAssetClient, headAssetCacheLimits } from '../../head-assets.mjs';

const response = (data: unknown, status = 200) => new Response(JSON.stringify({ message: '', data }), { status });
const asset = (id: string, sortOrder: number, html = `<meta name="x-${id}" content="ok">`) => ({ id, label: id, html, sortOrder, updatedAt: '2026-08-22T00:00:00Z' });

describe('server-only SEO head asset cache', () => {
  it('refreshes, preserves API order, and does not log source', async () => {
    let now = 0; const logger = { info: vi.fn(), warn: vi.fn() }; const fetchImpl = vi.fn().mockResolvedValue(response([asset('second', 2), asset('first', 1)]));
    const client = createHeadAssetClient({ endpoint: 'https://api.test/seo/head-assets/ducth-dev', fetchImpl, now: () => now, logger });
    expect(await client.getAssets()).toEqual([asset('second', 2), asset('first', 1)]);
    now = headAssetCacheLimits.freshMs - 1; await client.getAssets(); expect(fetchImpl).toHaveBeenCalledTimes(1);
    expect(JSON.stringify(logger)).not.toContain('<meta');
  });
  it('coalesces concurrent refreshes and fails open on malformed responses', async () => {
    let release!: (value: Response) => void; const pending = new Promise<Response>((resolve) => { release = resolve; }); const fetchImpl = vi.fn().mockReturnValue(pending);
    const client = createHeadAssetClient({ endpoint: 'https://api.test/assets', fetchImpl });
    const first = client.getAssets(); const second = client.getAssets(); release(response([])); expect(await first).toEqual([]); expect(await second).toEqual([]); expect(fetchImpl).toHaveBeenCalledTimes(1);
    fetchImpl.mockResolvedValue(response({ nope: true })); expect(await client.getAssets()).toEqual([]);
  });
  it('uses bounded stale data and drops it after five minutes', async () => {
    let now = 0; const fetchImpl = vi.fn().mockResolvedValueOnce(response([asset('one', 1)])).mockRejectedValue(new Error('down')); const client = createHeadAssetClient({ endpoint: 'https://api.test/assets', fetchImpl, now: () => now });
    expect(await client.getAssets()).toHaveLength(1); now = headAssetCacheLimits.freshMs + 1; expect(await client.getAssets()).toHaveLength(1); now = headAssetCacheLimits.staleMs + 1; expect(await client.getAssets()).toEqual([]);
  });
});
