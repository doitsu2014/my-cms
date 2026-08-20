const supportedSamplers = new Set([
  'always_on',
  'always_off',
  'traceidratio',
  'parentbased_always_on',
  'parentbased_always_off',
]);

export function readWebsiteTelemetryConfig(env = process.env) {
  if (env.ENABLED_OTLP_EXPORTER !== 'true') return { enabled: false };

  const endpoint = env.OTEL_EXPORTER_OTLP_TRACES_ENDPOINT?.trim();
  const sampler = (env.OTEL_TRACES_SAMPLER || 'parentbased_always_on')
    .trim()
    .toLowerCase();
  if (!endpoint || !supportedSamplers.has(sampler))
    return { enabled: false, invalid: true };

  try {
    const parsed = new URL(endpoint);
    if (!['http:', 'https:'].includes(parsed.protocol))
      return { enabled: false, invalid: true };
  } catch {
    return { enabled: false, invalid: true };
  }

  return {
    enabled: true,
    endpoint,
    sampler,
    serviceName: env.OTEL_SERVICE_NAME?.trim() || 'ducth-dev-website',
  };
}

export function boundedRoute(route) {
  const path = typeof route === 'string' ? route.split('?')[0] : '/';
  return (path || '/').slice(0, 256);
}
