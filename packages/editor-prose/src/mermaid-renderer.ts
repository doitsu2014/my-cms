import type { Mermaid } from 'mermaid';

export type MermaidFailureCategory =
  | 'empty'
  | 'unsupported'
  | 'syntax'
  | 'limit'
  | 'load'
  | 'render';

export type MermaidRenderResult =
  | { ok: true; svg: string }
  | { ok: false; category: MermaidFailureCategory };

const MAX_SOURCE_LENGTH = 50_000;
const MAX_EDGES = 500;
const DIRECTIVE = /^\s*%%\{[\s\S]*?\}%%/m;
const FRONTMATTER = /^\s*---\s*(?:\r?\n|$)/;
let loader: Promise<Mermaid> | undefined;
let queue = Promise.resolve();
let renderCount = 0;

export const validateMermaidSource = (source: string): { ok: true } | { ok: false; category: MermaidFailureCategory } => {
  if (!source.trim()) return { ok: false, category: 'empty' };
  if (source.length > MAX_SOURCE_LENGTH || (source.match(/-->|==>|-\.->|<-->|<==>/g)?.length ?? 0) > MAX_EDGES) {
    return { ok: false, category: 'limit' };
  }
  if (DIRECTIVE.test(source) || FRONTMATTER.test(source)) {
    return { ok: false, category: 'unsupported' };
  }
  return { ok: true };
};

const loadMermaid = (): Promise<Mermaid> => {
  if (typeof window === 'undefined' || typeof document === 'undefined') {
    return Promise.reject(new Error('Mermaid is only available in a browser'));
  }
  if (!loader) {
    loader = import('mermaid')
      .then(({ default: mermaid }) => {
        mermaid.initialize({
          startOnLoad: false,
          securityLevel: 'strict',
          htmlLabels: false,
          suppressErrorRendering: true,
          maxTextSize: MAX_SOURCE_LENGTH,
          maxEdges: MAX_EDGES,
          theme: 'neutral',
        });
        return mermaid;
      })
      .catch((error: unknown) => {
        loader = undefined;
        throw error;
      });
  }
  return loader;
};

const enqueue = <T>(job: () => Promise<T>): Promise<T> => {
  const result = queue.then(job, job);
  queue = result.then(() => undefined, () => undefined);
  return result;
};

export const renderMermaid = (source: string): Promise<MermaidRenderResult> => {
  const policy = validateMermaidSource(source);
  if (!policy.ok) return Promise.resolve(policy);

  return enqueue(async () => {
    let mermaid: Mermaid;
    try {
      mermaid = await loadMermaid();
    } catch {
      return { ok: false, category: 'load' };
    }

    try {
      const parsed = await mermaid.parse(source, { suppressErrors: true });
      if (!parsed) return { ok: false, category: 'syntax' };
    } catch {
      return { ok: false, category: 'syntax' };
    }

    const renderId = `editor-prose-mermaid-${++renderCount}`;
    const temporaryContainer = document.createElement('div');
    temporaryContainer.hidden = true;
    temporaryContainer.setAttribute('aria-hidden', 'true');
    document.body.append(temporaryContainer);

    try {
      const { svg } = await mermaid.render(renderId, source, temporaryContainer);
      return { ok: true, svg };
    } catch {
      return { ok: false, category: 'render' };
    } finally {
      temporaryContainer.remove();
    }
  });
};

export const parseMermaidSource = async (source: string): Promise<MermaidRenderResult | { ok: true }> => {
  const policy = validateMermaidSource(source);
  if (!policy.ok) return policy;
  try {
    const mermaid = await loadMermaid();
    return await mermaid.parse(source, { suppressErrors: true })
      ? { ok: true }
      : { ok: false, category: 'syntax' };
  } catch {
    return { ok: false, category: 'load' };
  }
};
