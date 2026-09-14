import { useEffect, useLayoutEffect, useRef, useState } from 'react';

export type QuickToolbarAction = 'image' | 'video' | 'code-block';

export interface QuickToolbarProps {
  position: { x: number; y: number };
  onAction: (action: QuickToolbarAction) => void;
  onClose: () => void;
}

const VIEWPORT_MARGIN = 12;

export function QuickToolbar({ position, onAction, onClose }: QuickToolbarProps) {
  const menuRef = useRef<HTMLDivElement>(null);
  const firstActionRef = useRef<HTMLButtonElement>(null);
  const [placement, setPlacement] = useState(position);

  useLayoutEffect(() => {
    const clamp = () => {
      const bounds = menuRef.current?.getBoundingClientRect();
      const width = bounds?.width ?? 0;
      const height = bounds?.height ?? 0;
      setPlacement({
        x: Math.max(VIEWPORT_MARGIN, Math.min(position.x, window.innerWidth - width - VIEWPORT_MARGIN)),
        y: Math.max(VIEWPORT_MARGIN, Math.min(position.y, window.innerHeight - height - VIEWPORT_MARGIN)),
      });
    };
    clamp();
    window.addEventListener('resize', clamp);
    return () => window.removeEventListener('resize', clamp);
  }, [position]);

  useEffect(() => {
    firstActionRef.current?.focus();
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') onClose();
    };
    const onPointerDown = (event: PointerEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) onClose();
    };
    document.addEventListener('keydown', onKeyDown);
    document.addEventListener('pointerdown', onPointerDown);
    return () => {
      document.removeEventListener('keydown', onKeyDown);
      document.removeEventListener('pointerdown', onPointerDown);
    };
  }, [onClose]);

  const select = (action: QuickToolbarAction) => {
    onAction(action);
    onClose();
  };

  return (
    <div
      ref={menuRef}
      role="menu"
      aria-label="Quick Toolbar"
      className="fixed z-[60] flex min-w-44 flex-col rounded-box border border-base-300 bg-base-100 p-1 shadow-lg"
      style={{ left: placement.x, top: placement.y }}
    >
      <button ref={firstActionRef} type="button" role="menuitem" className="btn btn-ghost btn-sm justify-start" onClick={() => select('image')}>Add Image</button>
      <button type="button" role="menuitem" className="btn btn-ghost btn-sm justify-start" onClick={() => select('video')}>Add Video</button>
      <button type="button" role="menuitem" className="btn btn-ghost btn-sm justify-start" onClick={() => select('code-block')}>Add Code Block</button>
    </div>
  );
}
