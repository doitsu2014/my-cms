import type { Node as ProseMirrorNode } from '@tiptap/pm/model';
import { Selection, type SelectionBookmark, type EditorState, type Transaction } from '@tiptap/pm/state';

/**
 * Owns a deferred editor selection while a menu, dialog, or upload moves focus.
 * The bookmark is mapped on every editor transaction before it is resolved.
 */
export class EditorInsertionTarget {
  private bookmark: SelectionBookmark | null;

  constructor(bookmark: SelectionBookmark | null = null) {
    this.bookmark = bookmark;
  }

  get hasTarget(): boolean {
    return this.bookmark !== null;
  }

  capture(bookmark: SelectionBookmark): void {
    this.bookmark = bookmark;
  }

  map(transaction: Transaction): void {
    if (this.bookmark) this.bookmark = this.bookmark.map(transaction.mapping);
  }

  clear(): void {
    this.bookmark = null;
  }

  restore(state: EditorState): Transaction | null {
    return this.bookmark ? state.tr.setSelection(this.bookmark.resolve(state.doc)) : null;
  }

  insertImages(state: EditorState, urls: string[]): Transaction | null {
    if (!this.bookmark || urls.length === 0) return null;
    const selection = this.bookmark.resolve(state.doc);
    const imageType = state.schema.nodes.image;
    if (!imageType) return null;

    let transaction = state.tr.setSelection(selection);
    let position = selection.from;
    if (!selection.empty) {
      transaction = transaction.deleteSelection();
      position = transaction.selection.from;
    }

    for (const url of urls) {
      const image = imageType.create({ src: url });
      transaction = transaction.insert(position, image);
      position += image.nodeSize;
    }

    this.clear();
    return transaction.setSelection(Selection.near(transaction.doc.resolve(position)));
  }

  insertYoutube(state: EditorState, url: string): Transaction | null {
    if (!this.bookmark || !url.trim()) return null;
    const selection = this.bookmark.resolve(state.doc);
    this.clear();
    return state.tr.setSelection(selection);
  }

  insertCodeBlock(state: EditorState): Transaction | null {
    if (!this.bookmark) return null;
    const selection = this.bookmark.resolve(state.doc);
    const codeBlock = state.schema.nodes.codeBlock;
    if (!codeBlock) return null;
    this.clear();
    return state.tr.replaceSelectionWith(codeBlock.create()).scrollIntoView();
  }

  resolve(state: EditorState): ProseMirrorNode | null {
    return this.bookmark ? this.bookmark.resolve(state.doc).$from.parent : null;
  }
}
