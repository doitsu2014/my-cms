const FRESH_MS = 60_000;
const STALE_MS = 5 * 60_000;
const REQUEST_TIMEOUT_MS = 2_000;

function parsePublicAssets(payload) {
  const assets = payload?.data;
  if (!Array.isArray(assets)) throw new Error('public SEO asset response has no data collection');
  return assets.map((asset) => {
    if (!asset || typeof asset !== 'object' || typeof asset.id !== 'string' || typeof asset.label !== 'string' || asset.label.length === 0 || typeof asset.html !== 'string' || asset.html.length === 0 || asset.html.length > 32768 || typeof asset.sortOrder !== 'number' || !Number.isInteger(asset.sortOrder) || asset.sortOrder <= 0 || typeof asset.updatedAt !== 'string') {
      throw new Error('public SEO asset response has an invalid item');
    }
    return { id: asset.id, label: asset.label, html: asset.html, sortOrder: asset.sortOrder, updatedAt: asset.updatedAt };
  });
}

export function createHeadAssetClient({ endpoint, fetchImpl = fetch, now = () => Date.now(), logger = console } = {}) {
  let successful = null;
  let refreshInFlight = null;

  const refresh = async () => {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetchImpl(endpoint, { signal: controller.signal, headers: { Accept: 'application/json' } });
      if (!response.ok) throw new Error(`public SEO asset endpoint returned ${response.status}`);
      const assets = parsePublicAssets(await response.json());
      successful = { assets, fetchedAt: now() };
      logger.info?.('seo_head_assets_refresh_success', { count: assets.length });
      return assets;
    } catch (error) {
      logger.warn?.('seo_head_assets_refresh_failed', { reason: error instanceof Error ? error.name : 'unknown' });
      throw error;
    } finally {
      clearTimeout(timeout);
    }
  };

  const getAssets = async () => {
    const currentTime = now();
    if (successful && currentTime - successful.fetchedAt < FRESH_MS) return successful.assets;
    if (!refreshInFlight) refreshInFlight = refresh().finally(() => { refreshInFlight = null; });
    try {
      return await refreshInFlight;
    } catch {
      if (successful && currentTime - successful.fetchedAt <= STALE_MS) {
        logger.warn?.('seo_head_assets_stale_fallback', { ageMs: currentTime - successful.fetchedAt });
        return successful.assets;
      }
      successful = null;
      return [];
    }
  };

  return { getAssets };
}

export const headAssetCacheLimits = { freshMs: FRESH_MS, staleMs: STALE_MS, requestTimeoutMs: REQUEST_TIMEOUT_MS };
