import {
  context,
  propagation,
  ROOT_CONTEXT,
  SpanKind,
  SpanStatusCode,
  trace,
} from '@opentelemetry/api';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http';
import { W3CTraceContextPropagator } from '@opentelemetry/core';
import { resourceFromAttributes } from '@opentelemetry/resources';
import { NodeSDK } from '@opentelemetry/sdk-node';
import {
  AlwaysOffSampler,
  AlwaysOnSampler,
  ParentBasedSampler,
  TraceIdRatioBasedSampler,
} from '@opentelemetry/sdk-trace-base';
import {
  boundedRoute,
  readWebsiteTelemetryConfig,
} from './telemetry-config.mjs';

const telemetryConfig = readWebsiteTelemetryConfig();
const enabled = telemetryConfig.enabled;
let sdk;
let shutdownStarted = false;

const warn = (message) => {
  console.warn(`[website-otel] ${message}`);
};

const parseSampler = (value) => {
  switch ((value || 'parentbased_always_on').trim().toLowerCase()) {
    case 'always_on':
      return new AlwaysOnSampler();
    case 'always_off':
      return new AlwaysOffSampler();
    case 'traceidratio':
      return new TraceIdRatioBasedSampler(0.1);
    case 'parentbased_always_on':
      return new ParentBasedSampler({ root: new AlwaysOnSampler() });
    case 'parentbased_always_off':
      return new ParentBasedSampler({ root: new AlwaysOffSampler() });
    default:
      return undefined;
  }
};

const headersGetter = {
  get(carrier, key) {
    const value = carrier[key] ?? carrier[key.toLowerCase()];
    return Array.isArray(value) ? value[0] : value;
  },
  keys(carrier) {
    return Object.keys(carrier);
  },
};

const headerSetter = {
  set(carrier, key, value) {
    carrier.set(key, value);
  },
};

const safeProtocol = (protocol) =>
  typeof protocol === 'string' && protocol.length <= 16 ? protocol : 'http';

function installBridge() {
  const tracer = trace.getTracer('ducth-dev-website');
  globalThis.__WEBSITE_OTEL_BRIDGE__ = {
    runServerSpan(input, callback) {
      const parent = propagation.extract(
        ROOT_CONTEXT,
        input.headers,
        headersGetter,
      );
      return context.with(parent, () =>
        tracer.startActiveSpan(
          'website.ssr.request',
          {
            kind: SpanKind.SERVER,
            attributes: {
              'http.method': input.method.slice(0, 16),
              'http.route': boundedRoute(input.route),
              'network.protocol.version': safeProtocol(input.protocol),
            },
          },
          async (span) => {
            let statusCode = 200;
            let errorCategory;
            const handle = {
              setStatus(status) {
                if (Number.isInteger(status)) statusCode = status;
              },
              setErrorCategory(category) {
                errorCategory = category;
              },
            };
            try {
              return await context.with(
                trace.setSpan(context.active(), span),
                () => callback(handle),
              );
            } catch (error) {
              errorCategory ||= 'ssr';
              throw error;
            } finally {
              span.setAttribute('http.status_code', statusCode);
              if (errorCategory) {
                span.setAttribute('error.type', errorCategory);
                span.setStatus({ code: SpanStatusCode.ERROR });
              }
              span.end();
            }
          },
        ),
      );
    },
    tracedFetch(input, init) {
      const source = input instanceof Request ? input : undefined;
      const url = new URL(source?.url ?? String(input));
      const method = (init?.method ?? source?.method ?? 'GET').toUpperCase();
      const tracerInput = {
        'http.method': method.slice(0, 16),
        'server.address': url.origin.slice(0, 256),
        'url.path': (url.pathname || '/').slice(0, 256),
      };
      return tracer.startActiveSpan(
        'website.graphql.client',
        { kind: SpanKind.CLIENT, attributes: tracerInput },
        async (span) => {
          const headers = new Headers(init?.headers ?? source?.headers);
          propagation.inject(context.active(), headers, headerSetter);
          const request = source
            ? new Request(source, { ...init, headers })
            : new Request(url, { ...init, headers });
          try {
            const response = await context.with(
              trace.setSpan(context.active(), span),
              () => fetch(request),
            );
            span.setAttribute('http.status_code', response.status);
            if (response.status >= 500)
              span.setStatus({ code: SpanStatusCode.ERROR });
            return response;
          } catch (error) {
            span.setAttribute('error.type', 'graphql');
            span.setStatus({ code: SpanStatusCode.ERROR });
            throw error;
          } finally {
            span.end();
          }
        },
      );
    },
  };
}

if (enabled) {
  const endpoint = telemetryConfig.endpoint;
  const serviceName = telemetryConfig.serviceName;
  const sampler = parseSampler(telemetryConfig.sampler);
  if (!endpoint || !sampler) {
    warn('invalid telemetry configuration; tracing disabled');
  } else {
    try {
      sdk = new NodeSDK({
        resource: resourceFromAttributes({ 'service.name': serviceName }),
        traceExporter: new OTLPTraceExporter({ url: endpoint }),
        sampler,
        textMapPropagator: new W3CTraceContextPropagator(),
        instrumentations: [],
      });
      sdk.start();
      installBridge();
    } catch {
      sdk = undefined;
      warn('telemetry initialization failed; tracing disabled');
    }
  }
}

async function shutdown() {
  if (shutdownStarted) return;
  shutdownStarted = true;
  if (!sdk) return;
  try {
    await sdk.shutdown();
  } catch {
    warn('telemetry shutdown failed');
  }
}

process.once('SIGTERM', shutdown);
process.once('SIGINT', shutdown);
