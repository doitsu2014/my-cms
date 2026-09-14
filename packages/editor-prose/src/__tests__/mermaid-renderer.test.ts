import { describe, expect, it } from 'vitest';
import { parseMermaidSource, validateMermaidSource } from '../mermaid-renderer';

describe('Mermaid renderer policy', () => {
  it('accepts ordinary flowchart and sequence source', () => {
    expect(validateMermaidSource('flowchart LR\n  A --> B')).toEqual({ ok: true });
    expect(validateMermaidSource('sequenceDiagram\n  Alice->>Bob: Hello')).toEqual({ ok: true });
  });

  it('parses ordinary flowchart and sequence source with the pinned runtime', async () => {
    await expect(parseMermaidSource('flowchart LR\n  A --> B')).resolves.toEqual({ ok: true });
    await expect(parseMermaidSource('sequenceDiagram\n  Alice->>Bob: Hello')).resolves.toEqual({ ok: true });
  });

  it('rejects author configuration before the runtime is loaded', () => {
    expect(validateMermaidSource('%%{init: { "securityLevel": "loose" }}%%\nflowchart LR\nA-->B')).toMatchObject({ ok: false, category: 'unsupported' });
    expect(validateMermaidSource('---\nconfig:\n  theme: dark\n---\nflowchart LR\nA-->B')).toMatchObject({ ok: false, category: 'unsupported' });
  });

  it('rejects empty and oversized source locally', () => {
    expect(validateMermaidSource(' \n')).toMatchObject({ ok: false, category: 'empty' });
    expect(validateMermaidSource('a'.repeat(50_001))).toMatchObject({ ok: false, category: 'limit' });
  });
});
