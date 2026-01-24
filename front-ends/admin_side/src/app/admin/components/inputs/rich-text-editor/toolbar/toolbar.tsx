import React, { useCallback, useState, useRef } from 'react';
import type { Editor } from '@tiptap/react';
import { toast } from 'sonner';
import TurndownService from 'turndown';
import { marked } from 'marked';
import { getMediaUploadApiUrl, getMediaImageUrl, createAuthHeaders } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';
import {
  Bold,
  Italic,
  Underline,
  Strikethrough,
  Code,
  Code2,
  Heading1,
  Heading2,
  Heading3,
  Heading4,
  Heading5,
  Heading6,
  List,
  ListOrdered,
  CheckSquare,
  AlignLeft,
  AlignCenter,
  AlignRight,
  AlignJustify,
  Quote,
  Link,
  Unlink,
  Image,
  Youtube,
  Table,
  TableProperties,
  Rows3,
  Columns3,
  Trash2,
  Subscript,
  Superscript,
  Maximize,
  Minimize,
  FileCode,
  RemoveFormatting,
  Eye,
  EyeOff,
  Palette,
  Highlighter,
  ChevronDown,
  Plus,
  Minus,
  CornerDownLeft,
  Upload,
  FileDown,
  FileUp,
} from 'lucide-react';

// Initialize Turndown service for HTML to Markdown conversion
const turndownService = new TurndownService({
  headingStyle: 'atx',
  codeBlockStyle: 'fenced',
  emDelimiter: '*',
});

// Add custom rule for code blocks with language
turndownService.addRule('codeBlock', {
  filter: (node) => {
    return node.nodeName === 'PRE' && !!(node as HTMLElement).querySelector('code');
  },
  replacement: (content, node) => {
    const codeElement = (node as HTMLElement).querySelector('code');
    const language = codeElement?.className?.match(/language-(\w+)/)?.[1] || '';
    const code = codeElement?.textContent || content;
    return `\n\`\`\`${language}\n${code}\n\`\`\`\n`;
  },
});

// Configure marked for Markdown to HTML conversion
marked.setOptions({
  gfm: true,
  breaks: false,
});

interface ToolbarProps {
  editor: Editor;
  onToggleFullscreen: () => void;
  isFullscreen: boolean;
  onOpenHtmlEditor: () => void;
  onTogglePreview: () => void;
  isPreview: boolean;
}

interface ToolbarButtonProps {
  onClick: () => void;
  isActive?: boolean;
  disabled?: boolean;
  title: string;
  children: React.ReactNode;
}

const ToolbarButton: React.FC<ToolbarButtonProps> = ({
  onClick,
  isActive = false,
  disabled = false,
  title,
  children,
}) => (
  <div className="tooltip tooltip-bottom" data-tip={title}>
    <button
      type="button"
      onClick={onClick}
      disabled={disabled}
      className={`btn btn-ghost btn-xs h-8 min-h-8 px-2 ${
        isActive ? 'bg-primary/20 text-primary' : ''
      }`}
    >
      {children}
    </button>
  </div>
);

const ToolbarDivider: React.FC = () => (
  <div className="w-px h-6 bg-base-300 mx-1" />
);

export const Toolbar: React.FC<ToolbarProps> = ({
  editor,
  onToggleFullscreen,
  isFullscreen,
  onOpenHtmlEditor,
  onTogglePreview,
  isPreview,
}) => {
  const { token } = useAuth();
  const [showLinkInput, setShowLinkInput] = useState(false);
  const [linkUrl, setLinkUrl] = useState('');
  const [showImageInput, setShowImageInput] = useState(false);
  const [imageUrl, setImageUrl] = useState('');
  const [showYoutubeInput, setShowYoutubeInput] = useState(false);
  const [youtubeUrl, setYoutubeUrl] = useState('');
  const [isUploading, setIsUploading] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [showMarkdownModal, setShowMarkdownModal] = useState(false);
  const [markdownInput, setMarkdownInput] = useState('');

  // Link handlers
  const setLink = useCallback(() => {
    if (linkUrl) {
      editor
        .chain()
        .focus()
        .extendMarkRange('link')
        .setLink({ href: linkUrl })
        .run();
    }
    setLinkUrl('');
    setShowLinkInput(false);
  }, [editor, linkUrl]);

  const unsetLink = useCallback(() => {
    editor.chain().focus().unsetLink().run();
  }, [editor]);

  // Image handler
  const addImage = useCallback(() => {
    if (imageUrl) {
      editor.chain().focus().setImage({ src: imageUrl }).run();
    }
    setImageUrl('');
    setShowImageInput(false);
  }, [editor, imageUrl]);

  // Image upload handler
  const handleImageUpload = useCallback(async (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (!files || files.length === 0) return;

    setIsUploading(true);
    try {
      for (const file of Array.from(files)) {
        const formData = new FormData();
        formData.append('image', file);

        const response = await fetch(getMediaUploadApiUrl(), {
          method: 'POST',
          headers: createAuthHeaders(token),
          body: formData
        });

        if (!response.ok) {
          throw new Error('Image upload failed');
        }

        const data = await response.json();
        // Construct the image URL from the path returned by the API
        const uploadedUrl = getMediaImageUrl(data.data.path);

        // Insert the uploaded image into the editor
        editor.chain().focus().setImage({ src: uploadedUrl }).run();
      }
      toast.success('Image uploaded successfully');
    } catch (error) {
      console.error('Error uploading image:', error);
      toast.error('Image upload failed. Please try again.');
    } finally {
      setIsUploading(false);
      // Reset file input
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    }
  }, [editor, token]);

  // YouTube handler
  const addYoutube = useCallback(() => {
    if (youtubeUrl) {
      editor.chain().focus().setYoutubeVideo({ src: youtubeUrl }).run();
    }
    setYoutubeUrl('');
    setShowYoutubeInput(false);
  }, [editor, youtubeUrl]);

  // Color picker state
  const [showColorPicker, setShowColorPicker] = useState(false);
  const [showHighlightPicker, setShowHighlightPicker] = useState(false);

  // Import markdown handler
  const importMarkdown = useCallback(async () => {
    if (!markdownInput.trim()) {
      toast.error('Please enter some Markdown content');
      return;
    }
    try {
      let html = await marked(markdownInput);

      // Clean up extra whitespace in code blocks
      html = html.replace(/<pre><code([^>]*)>\n+/g, '<pre><code$1>');
      html = html.replace(/\n+<\/code><\/pre>/g, '</code></pre>');
      // Remove multiple consecutive empty paragraphs
      html = html.replace(/(<p>\s*<\/p>\s*)+/g, '');

      editor.commands.setContent(html);
      setShowMarkdownModal(false);
      setMarkdownInput('');
      toast.success('Markdown imported successfully');
    } catch (error) {
      console.error('Error converting Markdown:', error);
      toast.error('Failed to import Markdown');
    }
  }, [editor, markdownInput]);

  const colors = [
    '#000000', '#434343', '#666666', '#999999', '#b7b7b7', '#cccccc', '#d9d9d9', '#efefef', '#f3f3f3', '#ffffff',
    '#980000', '#ff0000', '#ff9900', '#ffff00', '#00ff00', '#00ffff', '#4a86e8', '#0000ff', '#9900ff', '#ff00ff',
    '#e6b8af', '#f4cccc', '#fce5cd', '#fff2cc', '#d9ead3', '#d0e0e3', '#c9daf8', '#cfe2f3', '#d9d2e9', '#ead1dc',
  ];

  return (
    <div className="tiptap-toolbar flex flex-wrap items-center gap-0.5 p-2 border-b border-base-300 bg-base-100 rounded-t-lg">
      {/* Headings Dropdown */}
      <div className="dropdown tooltip tooltip-bottom" data-tip="Heading">
        <div tabIndex={0} role="button" className="btn btn-ghost btn-xs h-8 min-h-8 gap-1">
          <span className="text-xs">Heading</span>
          <ChevronDown size={12} />
        </div>
        <ul tabIndex={0} className="dropdown-content menu bg-base-200 rounded-box z-50 w-40 p-2 shadow">
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setParagraph().run()}
              className={editor.isActive('paragraph') ? 'active' : ''}
            >
              Normal
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
              className={editor.isActive('heading', { level: 1 }) ? 'active' : ''}
            >
              <Heading1 size={16} /> Heading 1
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
              className={editor.isActive('heading', { level: 2 }) ? 'active' : ''}
            >
              <Heading2 size={16} /> Heading 2
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().toggleHeading({ level: 3 }).run()}
              className={editor.isActive('heading', { level: 3 }) ? 'active' : ''}
            >
              <Heading3 size={16} /> Heading 3
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().toggleHeading({ level: 4 }).run()}
              className={editor.isActive('heading', { level: 4 }) ? 'active' : ''}
            >
              <Heading4 size={16} /> Heading 4
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().toggleHeading({ level: 5 }).run()}
              className={editor.isActive('heading', { level: 5 }) ? 'active' : ''}
            >
              <Heading5 size={16} /> Heading 5
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().toggleHeading({ level: 6 }).run()}
              className={editor.isActive('heading', { level: 6 }) ? 'active' : ''}
            >
              <Heading6 size={16} /> Heading 6
            </button>
          </li>
        </ul>
      </div>

      <ToolbarDivider />

      {/* Text Formatting */}
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleBold().run()}
        isActive={editor.isActive('bold')}
        title="Bold"
      >
        <Bold size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleItalic().run()}
        isActive={editor.isActive('italic')}
        title="Italic"
      >
        <Italic size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleUnderline().run()}
        isActive={editor.isActive('underline')}
        title="Underline"
      >
        <Underline size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleStrike().run()}
        isActive={editor.isActive('strike')}
        title="Strikethrough"
      >
        <Strikethrough size={16} />
      </ToolbarButton>

      <ToolbarDivider />

      {/* Text Color */}
      <div className="dropdown tooltip tooltip-bottom" data-tip="Text Color">
        <div tabIndex={0} role="button" className="btn btn-ghost btn-xs h-8 min-h-8">
          <Palette size={16} />
        </div>
        <div tabIndex={0} className="dropdown-content bg-base-200 rounded-box z-50 p-3 shadow">
          <div className="grid grid-cols-10 gap-1">
            {colors.map((color) => (
              <button
                key={color}
                type="button"
                className="w-5 h-5 rounded border border-base-300 hover:scale-110 transition-transform"
                style={{ backgroundColor: color }}
                onClick={() => editor.chain().focus().setColor(color).run()}
              />
            ))}
          </div>
          <button
            type="button"
            className="btn btn-ghost btn-xs mt-2 w-full"
            onClick={() => editor.chain().focus().unsetColor().run()}
          >
            Remove color
          </button>
        </div>
      </div>

      {/* Highlight */}
      <div className="dropdown tooltip tooltip-bottom" data-tip="Highlight">
        <div tabIndex={0} role="button" className="btn btn-ghost btn-xs h-8 min-h-8">
          <Highlighter size={16} />
        </div>
        <div tabIndex={0} className="dropdown-content bg-base-200 rounded-box z-50 p-3 shadow">
          <div className="grid grid-cols-10 gap-1">
            {colors.map((color) => (
              <button
                key={color}
                type="button"
                className="w-5 h-5 rounded border border-base-300 hover:scale-110 transition-transform"
                style={{ backgroundColor: color }}
                onClick={() => editor.chain().focus().toggleHighlight({ color }).run()}
              />
            ))}
          </div>
          <button
            type="button"
            className="btn btn-ghost btn-xs mt-2 w-full"
            onClick={() => editor.chain().focus().unsetHighlight().run()}
          >
            Remove highlight
          </button>
        </div>
      </div>

      <ToolbarDivider />

      {/* Subscript/Superscript */}
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleSubscript().run()}
        isActive={editor.isActive('subscript')}
        title="Subscript"
      >
        <Subscript size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleSuperscript().run()}
        isActive={editor.isActive('superscript')}
        title="Superscript"
      >
        <Superscript size={16} />
      </ToolbarButton>

      <ToolbarDivider />

      {/* Lists */}
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleBulletList().run()}
        isActive={editor.isActive('bulletList')}
        title="Bullet List"
      >
        <List size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleOrderedList().run()}
        isActive={editor.isActive('orderedList')}
        title="Numbered List"
      >
        <ListOrdered size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleTaskList().run()}
        isActive={editor.isActive('taskList')}
        title="Task List"
      >
        <CheckSquare size={16} />
      </ToolbarButton>

      <ToolbarDivider />

      {/* Alignment */}
      <ToolbarButton
        onClick={() => editor.chain().focus().setTextAlign('left').run()}
        isActive={editor.isActive({ textAlign: 'left' })}
        title="Align Left"
      >
        <AlignLeft size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().setTextAlign('center').run()}
        isActive={editor.isActive({ textAlign: 'center' })}
        title="Align Center"
      >
        <AlignCenter size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().setTextAlign('right').run()}
        isActive={editor.isActive({ textAlign: 'right' })}
        title="Align Right"
      >
        <AlignRight size={16} />
      </ToolbarButton>
      <ToolbarButton
        onClick={() => editor.chain().focus().setTextAlign('justify').run()}
        isActive={editor.isActive({ textAlign: 'justify' })}
        title="Justify"
      >
        <AlignJustify size={16} />
      </ToolbarButton>

      <ToolbarDivider />

      {/* Block Elements */}
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleBlockquote().run()}
        isActive={editor.isActive('blockquote')}
        title="Blockquote"
      >
        <Quote size={16} />
      </ToolbarButton>

      {/* Inline Code */}
      <ToolbarButton
        onClick={() => editor.chain().focus().toggleCode().run()}
        isActive={editor.isActive('code')}
        title="Inline Code"
      >
        <Code size={16} />
      </ToolbarButton>

      {/* Code Block with Language Selection */}
      <div className="dropdown tooltip tooltip-bottom" data-tip="Code Block">
        <div
          tabIndex={0}
          role="button"
          className={`btn btn-ghost btn-xs h-8 min-h-8 gap-1 ${editor.isActive('codeBlock') ? 'bg-primary/20 text-primary' : ''}`}
        >
          <Code2 size={16} />
          <ChevronDown size={12} />
        </div>
        <ul tabIndex={0} className="dropdown-content menu bg-base-200 rounded-box z-50 w-44 p-2 shadow max-h-64 overflow-y-auto">
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().toggleCodeBlock().run()}
            >
              Plain Text
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'javascript' }).run()}
            >
              JavaScript
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'typescript' }).run()}
            >
              TypeScript
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'rust' }).run()}
            >
              Rust
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'csharp' }).run()}
            >
              C#
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'html' }).run()}
            >
              HTML
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'css' }).run()}
            >
              CSS
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'python' }).run()}
            >
              Python
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'java' }).run()}
            >
              Java
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'go' }).run()}
            >
              Go
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'sql' }).run()}
            >
              SQL
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'bash' }).run()}
            >
              Bash
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'json' }).run()}
            >
              JSON
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'yaml' }).run()}
            >
              YAML
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().setCodeBlock({ language: 'xml' }).run()}
            >
              XML
            </button>
          </li>
        </ul>
      </div>

      <ToolbarDivider />

      {/* Link */}
      <div className="dropdown tooltip tooltip-bottom" data-tip="Link">
        <div tabIndex={0} role="button" className={`btn btn-ghost btn-xs h-8 min-h-8 ${editor.isActive('link') ? 'bg-primary/20 text-primary' : ''}`}>
          <Link size={16} />
        </div>
        <div tabIndex={0} className="dropdown-content bg-base-200 rounded-box z-50 p-3 shadow w-72">
          <div className="flex gap-2">
            <input
              type="url"
              placeholder="https://example.com"
              className="input input-bordered input-sm flex-1"
              value={linkUrl}
              onChange={(e) => setLinkUrl(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && setLink()}
            />
            <button type="button" className="btn btn-primary btn-sm" onClick={setLink}>
              Add
            </button>
          </div>
        </div>
      </div>
      {editor.isActive('link') && (
        <ToolbarButton onClick={unsetLink} title="Remove Link">
          <Unlink size={16} />
        </ToolbarButton>
      )}

      {/* Image */}
      <div className="dropdown tooltip tooltip-bottom" data-tip="Image">
        <div tabIndex={0} role="button" className="btn btn-ghost btn-xs h-8 min-h-8">
          <Image size={16} />
        </div>
        <div tabIndex={0} className="dropdown-content bg-base-200 rounded-box z-50 p-3 shadow w-80">
          {/* File Upload */}
          <div className="mb-3">
            <label className="text-xs font-medium mb-1 block">Upload Image</label>
            <input
              ref={fileInputRef}
              type="file"
              accept="image/*"
              className="file-input file-input-bordered file-input-sm w-full"
              onChange={handleImageUpload}
              disabled={isUploading}
            />
            {isUploading && (
              <div className="flex items-center gap-2 mt-2 text-sm text-info">
                <span className="loading loading-spinner loading-xs"></span>
                Uploading...
              </div>
            )}
          </div>
          {/* URL Input */}
          <div className="divider text-xs my-2">OR</div>
          <label className="text-xs font-medium mb-1 block">Image URL</label>
          <div className="flex gap-2">
            <input
              type="url"
              placeholder="https://example.com/image.jpg"
              className="input input-bordered input-sm flex-1"
              value={imageUrl}
              onChange={(e) => setImageUrl(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && addImage()}
            />
            <button type="button" className="btn btn-primary btn-sm" onClick={addImage}>
              Add
            </button>
          </div>
        </div>
      </div>

      {/* YouTube */}
      <div className="dropdown tooltip tooltip-bottom" data-tip="YouTube Video">
        <div tabIndex={0} role="button" className="btn btn-ghost btn-xs h-8 min-h-8">
          <Youtube size={16} />
        </div>
        <div tabIndex={0} className="dropdown-content bg-base-200 rounded-box z-50 p-3 shadow w-72">
          <div className="flex gap-2">
            <input
              type="url"
              placeholder="YouTube URL"
              className="input input-bordered input-sm flex-1"
              value={youtubeUrl}
              onChange={(e) => setYoutubeUrl(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && addYoutube()}
            />
            <button type="button" className="btn btn-primary btn-sm" onClick={addYoutube}>
              Add
            </button>
          </div>
        </div>
      </div>

      <ToolbarDivider />

      {/* Table */}
      <div className="dropdown tooltip tooltip-bottom" data-tip="Table">
        <div tabIndex={0} role="button" className="btn btn-ghost btn-xs h-8 min-h-8">
          <Table size={16} />
        </div>
        <ul tabIndex={0} className="dropdown-content menu bg-base-200 rounded-box z-50 w-52 p-2 shadow">
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run()}
            >
              <Plus size={14} /> Insert Table (3x3)
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().addColumnBefore().run()}
              disabled={!editor.can().addColumnBefore()}
            >
              <Columns3 size={14} /> Add Column Before
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().addColumnAfter().run()}
              disabled={!editor.can().addColumnAfter()}
            >
              <Columns3 size={14} /> Add Column After
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().deleteColumn().run()}
              disabled={!editor.can().deleteColumn()}
            >
              <Minus size={14} /> Delete Column
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().addRowBefore().run()}
              disabled={!editor.can().addRowBefore()}
            >
              <Rows3 size={14} /> Add Row Before
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().addRowAfter().run()}
              disabled={!editor.can().addRowAfter()}
            >
              <Rows3 size={14} /> Add Row After
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().deleteRow().run()}
              disabled={!editor.can().deleteRow()}
            >
              <Minus size={14} /> Delete Row
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().mergeCells().run()}
              disabled={!editor.can().mergeCells()}
            >
              <TableProperties size={14} /> Merge Cells
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().splitCell().run()}
              disabled={!editor.can().splitCell()}
            >
              <TableProperties size={14} /> Split Cell
            </button>
          </li>
          <li>
            <button
              type="button"
              onClick={() => editor.chain().focus().deleteTable().run()}
              disabled={!editor.can().deleteTable()}
              className="text-error"
            >
              <Trash2 size={14} /> Delete Table
            </button>
          </li>
        </ul>
      </div>

      <ToolbarDivider />

      {/* Utilities */}
      <ToolbarButton
        onClick={() => editor.chain().focus().unsetAllMarks().clearNodes().run()}
        title="Clear Formatting"
      >
        <RemoveFormatting size={16} />
      </ToolbarButton>

      <ToolbarDivider />

      {/* Markdown Actions */}
      <ToolbarButton
        onClick={() => {
          const html = editor.getHTML();
          const markdown = turndownService.turndown(html);
          navigator.clipboard.writeText(markdown).then(() => {
            toast.success('Copied as Markdown');
          }).catch(() => {
            toast.error('Failed to copy');
          });
        }}
        title="Copy as Markdown"
      >
        <FileDown size={16} />
      </ToolbarButton>

      <ToolbarButton
        onClick={() => setShowMarkdownModal(true)}
        title="Import Markdown"
      >
        <FileUp size={16} />
      </ToolbarButton>

      {/* Markdown Import Modal */}
      {showMarkdownModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <div className="fixed inset-0 bg-black/50" onClick={() => setShowMarkdownModal(false)} />
          <div className="relative bg-base-100 rounded-lg shadow-xl w-full max-w-2xl mx-4 max-h-[80vh] flex flex-col">
            <div className="flex items-center justify-between p-4 border-b border-base-300">
              <h3 className="font-bold text-lg">Import Markdown</h3>
              <button
                type="button"
                className="btn btn-sm btn-circle btn-ghost"
                onClick={() => setShowMarkdownModal(false)}
              >
                ✕
              </button>
            </div>
            <div className="p-4 flex-1 overflow-auto">
              <textarea
                className="textarea textarea-bordered w-full h-64 font-mono text-sm"
                placeholder="Paste your Markdown content here..."
                value={markdownInput}
                onChange={(e) => setMarkdownInput(e.target.value)}
              />
            </div>
            <div className="flex justify-end gap-2 p-4 border-t border-base-300">
              <button
                type="button"
                className="btn btn-ghost"
                onClick={() => {
                  setShowMarkdownModal(false);
                  setMarkdownInput('');
                }}
              >
                Cancel
              </button>
              <button
                type="button"
                className="btn btn-primary"
                onClick={importMarkdown}
              >
                Import
              </button>
            </div>
          </div>
        </div>
      )}

      <ToolbarDivider />

      {/* Utilities */}
      <ToolbarButton onClick={onOpenHtmlEditor} title="Edit HTML">
        <FileCode size={16} />
      </ToolbarButton>

      <ToolbarButton
        onClick={onTogglePreview}
        title={isPreview ? 'Edit Mode' : 'Preview'}
        isActive={isPreview}
      >
        {isPreview ? <EyeOff size={16} /> : <Eye size={16} />}
      </ToolbarButton>

      <ToolbarButton onClick={onToggleFullscreen} title={isFullscreen ? 'Exit Fullscreen' : 'Fullscreen'}>
        {isFullscreen ? <Minimize size={16} /> : <Maximize size={16} />}
      </ToolbarButton>
    </div>
  );
};

export default Toolbar;
