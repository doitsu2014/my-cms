import { describe, expect, it } from 'vitest';
import { Schema } from '@tiptap/pm/model';
import { EditorState, TextSelection } from '@tiptap/pm/state';

import { EditorInsertionTarget } from './editor-insertion-target';

const schema = new Schema({
  nodes: {
    doc: { content: 'block+' },
    paragraph: { content: 'inline*', group: 'block' },
    text: { group: 'inline' },
    image: { group: 'inline', inline: true, attrs: { src: {} } },
    codeBlock: { content: 'text*', group: 'block', code: true },
  },
});

const stateWith = (text: string) => EditorState.create({
  schema,
  doc: schema.node('doc', null, [schema.node('paragraph', null, text ? [schema.text(text)] : [])]),
});

describe('EditorInsertionTarget', () => {
  it('maps a captured selection through a transaction and inserts images consecutively', () => {
    const initial = stateWith('ab');
    const target = new EditorInsertionTarget(initial.selection.getBookmark());
    const changed = initial.apply(initial.tr.insertText('x', 1));
    target.map(changed.tr);

    const transaction = target.insertImages(changed, ['first.png', 'second.png']);

    expect(transaction).toBeTruthy();
    expect(transaction?.doc.textContent).toBe('xab');
    const sources: string[] = [];
    transaction?.doc.descendants((node) => {
      if (node.type.name === 'image') sources.push(node.attrs.src as string);
    });
    expect(sources).toEqual(['first.png', 'second.png']);
    expect(target.hasTarget).toBe(false);
  });

  it('clears cancelled and completed targets', () => {
    const state = stateWith('text');
    const target = new EditorInsertionTarget(state.selection.getBookmark());
    target.clear();
    expect(target.hasTarget).toBe(false);

    target.capture(TextSelection.create(state.doc, 2).getBookmark());
    target.insertCodeBlock(state);
    expect(target.hasTarget).toBe(false);
  });
});
