import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { ArticleProse } from 'editor-prose';

const fixture = `<h2>Heading</h2><p>Paragraph with <u>underline</u>, <s>strikethrough</s>, and <mark>highlight</mark>.</p><ul><li>List item</li></ul><blockquote>Quote</blockquote><pre><code>const value = 1;</code></pre><ul data-type="taskList"><li data-type="taskItem"><label><input type="checkbox" /></label><div>Task item</div></li></ul><table class="tiptap-table"><tbody><tr><td>Cell</td></tr></tbody></table>`;

describe('editor-prose package', () => {
  it('renders the complete TipTap fixture through the shared package', () => {
    const { container } = render(<ArticleProse html={fixture} />);
    const prose = container.querySelector('.article-prose');

    expect(prose).toBeInTheDocument();
    expect(prose?.querySelector('h2')).toBeInTheDocument();
    expect(prose?.querySelector('p')).toBeInTheDocument();
    expect(prose?.querySelector('ul:not([data-type])')).toBeInTheDocument();
    expect(prose?.querySelector('blockquote')).toBeInTheDocument();
    expect(prose?.querySelector('pre')).toBeInTheDocument();
    expect(prose?.querySelector('li[data-type="taskItem"]')).toBeInTheDocument();
    expect(prose?.querySelector('u')).toBeInTheDocument();
    expect(prose?.querySelector('s')).toBeInTheDocument();
    expect(prose?.querySelector('mark')).toBeInTheDocument();
    expect(prose?.querySelector('table.tiptap-table')).toBeInTheDocument();
    expect(screen.getByText('Task item')).toBeInTheDocument();
  });
});
