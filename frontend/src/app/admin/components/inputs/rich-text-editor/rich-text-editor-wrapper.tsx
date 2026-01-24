import React from 'react';
import type { TipTapEditorProps } from './tiptap-editor';

// Lazy load the TipTap editor component for better performance
const TipTapEditor = React.lazy(() => import('./tiptap-editor'));

// Re-export types for backward compatibility
export type RichTextEditorProps = TipTapEditorProps;

/**
 * Wrapper component for the Rich Text Editor
 * Handles lazy loading with a suspense boundary
 */
export const RichTextEditorWrapper: React.FC<TipTapEditorProps> = (props) => {
  return (
    <React.Suspense
      fallback={
        <div className="flex justify-center items-center min-h-[400px] bg-base-200 rounded-lg">
          <div className="flex flex-col items-center gap-3">
            <span className="loading loading-spinner loading-lg text-primary"></span>
            <span className="text-sm text-base-content/70">Loading editor...</span>
          </div>
        </div>
      }
    >
      <TipTapEditor {...props} />
    </React.Suspense>
  );
};
