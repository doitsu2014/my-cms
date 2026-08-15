import { render } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { ArticleProse } from 'editor-prose';

const fixture = '<h2>Heading</h2><p>Paragraph</p><ul><li>List item</li></ul><blockquote>Quote</blockquote><pre><code>const value = 1;</code></pre><ul data-type="taskList"><li data-type="taskItem"><label><input type="checkbox"></label><div>Task item</div></li></ul><p><u>underline</u><s>strikethrough</s><mark>highlight</mark></p><table class="tiptap-table"><tbody><tr><td>Cell</td></tr></tbody></table>';

describe('editor-prose package', () => {
  it('serializes the same article wrapper and TipTap fixture as the reader', () => {
    const { container } = render(<ArticleProse html={fixture} />);

    expect(container.innerHTML).toBe(`<div class="article-prose">${fixture}</div>`);
  });
});
