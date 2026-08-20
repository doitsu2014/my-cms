/**
 * The website bundle is shared by the browser and Node SSR builds.  Keep the
 * OpenTelemetry implementation out of this module: the Node preload installs
 * a bridge on globalThis, while the browser sees the no-op fallback below.
 */

export type TelemetryAttributes = Record<string, string | number | boolean>;

export type ServerSpanHandle = {
  setStatus: (statusCode: number) => void;
  setErrorCategory: (category: 'ssr' | 'graphql' | 'exporter') => void;
};

type TelemetryBridge = {
  runServerSpan: <T>(
    input: {
      method: string;
      route: string;
      protocol: string;
      headers: Record<string, string | string[] | undefined>;
      peerAddress?: string;
    },
    callback: (span: ServerSpanHandle) => Promise<T>,
  ) => Promise<T>;
  tracedFetch: (
    input: RequestInfo | URL,
    init?: RequestInit,
  ) => Promise<Response>;
};

declare global {
  var __WEBSITE_OTEL_BRIDGE__: TelemetryBridge | undefined;
}

function bridge(): TelemetryBridge | undefined {
  return globalThis.__WEBSITE_OTEL_BRIDGE__;
}

export function withServerSpan<T>(
  input: {
    method: string;
    route: string;
    protocol: string;
    headers: Record<string, string | string[] | undefined>;
    peerAddress?: string;
  },
  callback: (span: ServerSpanHandle) => Promise<T>,
): Promise<T> {
  return (
    bridge()?.runServerSpan(input, callback) ??
    callback({
      setStatus: () => undefined,
      setErrorCategory: () => undefined,
    })
  );
}

export function getServerFetch(): typeof fetch {
  return bridge()?.tracedFetch ?? fetch;
}
