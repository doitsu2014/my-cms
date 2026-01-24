import React, { useEffect, useCallback, useState, useRef } from 'react';
import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import { Underline } from '@tiptap/extension-underline';
import { TextAlign } from '@tiptap/extension-text-align';
import { TextStyle } from '@tiptap/extension-text-style';
import { Color } from '@tiptap/extension-color';
import { Highlight } from '@tiptap/extension-highlight';
import { Link } from '@tiptap/extension-link';
import { Image } from '@tiptap/extension-image';
import { Youtube } from '@tiptap/extension-youtube';
import { Table } from '@tiptap/extension-table';
import { TableRow } from '@tiptap/extension-table-row';
import { TableCell } from '@tiptap/extension-table-cell';
import { TableHeader } from '@tiptap/extension-table-header';
import { Subscript } from '@tiptap/extension-subscript';
import { Superscript } from '@tiptap/extension-superscript';
import { Placeholder } from '@tiptap/extension-placeholder';
import { CodeBlockLowlight } from '@tiptap/extension-code-block-lowlight';
import { TaskList } from '@tiptap/extension-task-list';
import { TaskItem } from '@tiptap/extension-task-item';
import { all, createLowlight } from 'lowlight';
import ImageResize from 'tiptap-extension-resize-image';
import { toast } from 'sonner';
import { getMediaUploadApiUrl, getMediaImageUrl, createAuthHeaders } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';

import { Toolbar } from './toolbar/toolbar';
import './tiptap-editor.css';
import 'highlight.js/styles/github-dark.css';

// Create lowlight instance with all languages
const lowlight = createLowlight(all);

export interface TipTapEditorProps {
  key?: string;
  id?: string;
  readOnly?: boolean;
  defaultValue?: string;
  onTextChange?: (html: string) => void;
  onSelectionChange?: (...args: any[]) => void;
  className?: string;
  placeholder?: string;
}

export const TipTapEditor: React.FC<TipTapEditorProps> = ({
  id,
  readOnly = false,
  defaultValue = '',
  onTextChange,
  className,
  placeholder = 'Start writing...',
}) => {
  const { token } = useAuth();
  const [isUploadingPaste, setIsUploadingPaste] = useState(false);
  const tokenRef = useRef(token);

  // Keep token ref up to date
  useEffect(() => {
    tokenRef.current = token;
  }, [token]);

  // Upload image function using ref to avoid stale closure
  const uploadImage = useCallback(async (file: File): Promise<string | null> => {
    try {
      const formData = new FormData();
      formData.append('image', file);

      const response = await fetch(getMediaUploadApiUrl(), {
        method: 'POST',
        headers: createAuthHeaders(tokenRef.current),
        body: formData
      });

      if (!response.ok) {
        throw new Error('Image upload failed');
      }

      const data = await response.json();
      // Construct the image URL from the path returned by the API
      return getMediaImageUrl(data.data.path);
    } catch (error) {
      console.error('Error uploading image:', error);
      toast.error('Image upload failed');
      return null;
    }
  }, []);

  const editor = useEditor({
    extensions: [
      StarterKit.configure({
        heading: {
          levels: [1, 2, 3, 4, 5, 6],
        },
        codeBlock: false, // We use CodeBlockLowlight instead
      }),
      Underline,
      TextAlign.configure({
        types: ['heading', 'paragraph'],
      }),
      TextStyle,
      Color,
      Highlight.configure({
        multicolor: true,
      }),
      Link.configure({
        openOnClick: false,
        HTMLAttributes: {
          class: 'text-primary underline',
        },
      }),
      Image.configure({
        HTMLAttributes: {
          class: 'rounded max-w-full h-auto',
        },
      }),
      ImageResize,
      Youtube.configure({
        width: 640,
        height: 360,
        HTMLAttributes: {
          class: 'rounded max-w-full',
        },
      }),
      Table.configure({
        resizable: true,
        HTMLAttributes: {
          class: 'tiptap-table',
        },
      }),
      TableRow,
      TableCell,
      TableHeader,
      Subscript,
      Superscript,
      Placeholder.configure({
        placeholder,
      }),
      CodeBlockLowlight.configure({
        lowlight,
        HTMLAttributes: {
          class: 'code-block',
        },
      }),
      TaskList,
      TaskItem.configure({
        nested: true,
      }),
    ],
    content: defaultValue,
    editable: !readOnly,
    onUpdate: ({ editor }) => {
      if (onTextChange) {
        onTextChange(editor.getHTML());
      }
    },
    editorProps: {
      attributes: {
        class: 'tiptap-editor-content focus:outline-none',
      },
      handlePaste: (view, event) => {
        const items = event.clipboardData?.items;
        if (!items) return false;

        for (const item of Array.from(items)) {
          if (item.type.startsWith('image/')) {
            event.preventDefault();
            const file = item.getAsFile();
            if (file) {
              setIsUploadingPaste(true);
              toast.loading('Uploading image...', { id: 'paste-upload' });

              uploadImage(file).then((url) => {
                setIsUploadingPaste(false);
                toast.dismiss('paste-upload');

                if (url) {
                  view.dispatch(
                    view.state.tr.replaceSelectionWith(
                      view.state.schema.nodes.image.create({ src: url })
                    )
                  );
                  toast.success('Image uploaded successfully');
                }
              });
            }
            return true;
          }
        }
        return false;
      },
      handleDrop: (view, event, _slice, moved) => {
        if (moved) return false;

        const files = event.dataTransfer?.files;
        if (!files || files.length === 0) return false;

        for (const file of Array.from(files)) {
          if (file.type.startsWith('image/')) {
            event.preventDefault();
            setIsUploadingPaste(true);
            toast.loading('Uploading image...', { id: 'drop-upload' });

            const coordinates = view.posAtCoords({ left: event.clientX, top: event.clientY });

            uploadImage(file).then((url) => {
              setIsUploadingPaste(false);
              toast.dismiss('drop-upload');

              if (url && coordinates) {
                const node = view.state.schema.nodes.image.create({ src: url });
                const transaction = view.state.tr.insert(coordinates.pos, node);
                view.dispatch(transaction);
                toast.success('Image uploaded successfully');
              }
            });
            return true;
          }
        }
        return false;
      },
    },
  });

  // Update content when defaultValue changes
  useEffect(() => {
    if (editor && defaultValue !== undefined) {
      const currentContent = editor.getHTML();
      // Only update if content actually differs (avoid cursor jump)
      if (currentContent !== defaultValue && defaultValue !== '<p></p>') {
        editor.commands.setContent(defaultValue, { emitUpdate: false });
      }
    }
  }, [editor, defaultValue]);

  // Update editable state when readOnly changes
  useEffect(() => {
    if (editor) {
      editor.setEditable(!readOnly);
    }
  }, [editor, readOnly]);

  // Fullscreen toggle handler
  const [isFullscreen, setIsFullscreen] = React.useState(false);

  const toggleFullscreen = useCallback(() => {
    setIsFullscreen((prev) => !prev);
  }, []);

  // Preview modal state
  const [showPreviewModal, setShowPreviewModal] = React.useState(false);

  const togglePreview = useCallback(() => {
    setShowPreviewModal((prev) => !prev);
  }, []);

  // HTML edit modal state
  const [showHtmlModal, setShowHtmlModal] = React.useState(false);
  const [htmlContent, setHtmlContent] = React.useState('');

  const openHtmlEditor = useCallback(() => {
    if (editor) {
      setHtmlContent(editor.getHTML());
      setShowHtmlModal(true);
    }
  }, [editor]);

  const saveHtmlContent = useCallback(() => {
    if (editor) {
      editor.commands.setContent(htmlContent, { emitUpdate: false });
      setShowHtmlModal(false);
      if (onTextChange) {
        onTextChange(htmlContent);
      }
    }
  }, [editor, htmlContent, onTextChange]);

  if (!editor) {
    return null;
  }

  return (
    <div
      className={`tiptap-container ${isFullscreen ? 'tiptap-fullscreen' : ''} ${className || ''}`}
      id={id}
    >
      <Toolbar
        editor={editor}
        onToggleFullscreen={toggleFullscreen}
        isFullscreen={isFullscreen}
        onOpenHtmlEditor={openHtmlEditor}
        onTogglePreview={togglePreview}
        isPreview={showPreviewModal}
      />
      <EditorContent editor={editor} />

      {/* Preview Modal */}
      {showPreviewModal && (
        <div className="modal modal-open">
          <div className="modal-box max-w-5xl w-[90vw] h-[90vh] flex flex-col">
            <div className="flex justify-between items-center mb-4">
              <h3 className="font-bold text-lg">Article Preview</h3>
              <button
                type="button"
                className="btn btn-sm btn-circle btn-ghost"
                onClick={() => setShowPreviewModal(false)}
              >
                ✕
              </button>
            </div>
            <div
              className="tiptap-preview flex-1 overflow-auto p-6 bg-base-200 rounded-lg"
              dangerouslySetInnerHTML={{ __html: editor.getHTML() }}
            />
            <div className="modal-action">
              <button
                type="button"
                className="btn btn-primary"
                onClick={() => setShowPreviewModal(false)}
              >
                Close
              </button>
            </div>
          </div>
          <div className="modal-backdrop" onClick={() => setShowPreviewModal(false)} />
        </div>
      )}

      {/* HTML Edit Modal */}
      {showHtmlModal && (
        <div className="modal modal-open">
          <div className="modal-box max-w-4xl h-[80vh]">
            <h3 className="font-bold text-lg mb-4">Edit HTML</h3>
            <textarea
              className="textarea textarea-bordered w-full h-[calc(100%-8rem)] font-mono text-sm"
              value={htmlContent}
              onChange={(e) => setHtmlContent(e.target.value)}
            />
            <div className="modal-action">
              <button
                type="button"
                className="btn btn-ghost"
                onClick={() => setShowHtmlModal(false)}
              >
                Cancel
              </button>
              <button
                type="button"
                className="btn btn-primary"
                onClick={saveHtmlContent}
              >
                Save
              </button>
            </div>
          </div>
          <div className="modal-backdrop" onClick={() => setShowHtmlModal(false)} />
        </div>
      )}
    </div>
  );
};

export default TipTapEditor;
