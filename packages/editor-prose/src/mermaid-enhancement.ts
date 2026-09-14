import { renderMermaid } from './mermaid-renderer';

export interface MermaidPresentationLabels {
  loading: string;
  failure: string;
  source: string;
  image: string;
}

export const defaultMermaidPresentationLabels: MermaidPresentationLabels = {
  loading: 'Rendering diagram…',
  failure: 'Diagram could not be rendered. Source remains available.',
  source: 'Diagram source (Mermaid)',
  image: 'Mermaid diagram. Original source is available in the Diagram source disclosure.',
};

const isMermaidCode = (code: Element): code is HTMLElement =>
  code instanceof HTMLElement && code.parentElement?.tagName === 'PRE' && code.classList.contains('language-mermaid');

const makeSourceDisclosure = (sourcePre: HTMLElement, labels: MermaidPresentationLabels, open: boolean): HTMLDetailsElement => {
  const details = document.createElement('details');
  details.className = 'mermaid-diagram__source';
  details.open = open;
  const summary = document.createElement('summary');
  summary.textContent = labels.source;
  details.append(summary, sourcePre);
  return details;
};

export const enhanceMermaidBlocks = (root: HTMLElement, labels: MermaidPresentationLabels): (() => void) => {
  let active = true;
  const cleanups: Array<() => void> = [];
  const blocks = Array.from(root.querySelectorAll('pre > code')).filter(isMermaidCode);

  for (const code of blocks) {
    const sourcePre = code.parentElement as HTMLElement;
    const source = code.textContent ?? '';
    const shell = document.createElement('section');
    shell.className = 'mermaid-diagram';
    const status = document.createElement('p');
    status.className = 'mermaid-diagram__status';
    status.role = 'status';
    status.setAttribute('aria-live', 'polite');
    status.textContent = labels.loading;
    shell.append(status);
    sourcePre.insertAdjacentElement('afterend', shell);

    let blobUrl: string | undefined;
    let timeout: ReturnType<typeof setTimeout> | undefined;
    let timedOut = false;

    const restoreSource = () => {
      if (shell.contains(sourcePre)) shell.before(sourcePre);
      if (blobUrl) URL.revokeObjectURL(blobUrl);
      shell.remove();
    };
    cleanups.push(() => {
      active = false;
      if (timeout) clearTimeout(timeout);
      restoreSource();
    });

    timeout = setTimeout(() => {
      if (!active || !sourcePre.isConnected) return;
      timedOut = true;
      status.textContent = labels.failure;
      shell.append(makeSourceDisclosure(sourcePre, labels, true));
    }, 10_000);

    void renderMermaid(source).then((result) => {
      if (!active || timedOut || !sourcePre.isConnected) return;
      if (timeout) clearTimeout(timeout);

      if (!result.ok) {
        status.textContent = labels.failure;
        shell.append(makeSourceDisclosure(sourcePre, labels, true));
        return;
      }

      const image = document.createElement('img');
      image.className = 'mermaid-diagram__image';
      image.alt = labels.image;
      image.src = URL.createObjectURL(new Blob([result.svg], { type: 'image/svg+xml' }));
      blobUrl = image.src;
      const sourceDisclosure = makeSourceDisclosure(sourcePre, labels, false);
      image.addEventListener('load', () => {
        if (active) status.remove();
      }, { once: true });
      image.addEventListener('error', () => {
        if (!active) return;
        image.remove();
        if (blobUrl) URL.revokeObjectURL(blobUrl);
        blobUrl = undefined;
        sourceDisclosure.open = true;
        status.textContent = labels.failure;
      }, { once: true });
      shell.append(image, sourceDisclosure);
    }).catch(() => {
      if (!active || timedOut || !sourcePre.isConnected) return;
      if (timeout) clearTimeout(timeout);
      status.textContent = labels.failure;
      shell.append(makeSourceDisclosure(sourcePre, labels, true));
    });
  }

  return () => {
    active = false;
    cleanups.forEach((cleanup) => cleanup());
  };
};
