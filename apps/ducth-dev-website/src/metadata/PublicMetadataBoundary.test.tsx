import '@testing-library/jest-dom/vitest';
import { MockedProvider } from '@apollo/client/testing';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter, Link, Route, Routes } from 'react-router-dom';
import { expect, it } from 'vitest';
import PublicMetadataBoundary from './PublicMetadataBoundary';

const config = {
  siteName: 'Test Site',
  siteUrl: 'https://example.test',
  defaultTitle: 'Writing index',
  defaultDescription: 'Notes about software and systems.',
  defaultLocale: 'en',
  graphqlApiUrl: 'https://example.test/graphql',
  graphqlCacheApiUrl: 'https://example.test/graphql',
  mediaBaseUrl: 'https://example.test/media',
  port: '3001',
};

it('reconciles managed head tags after browser navigation', async () => {
  document.body.innerHTML = `<script id="app-config" type="application/json">${JSON.stringify(config)}</script>`;
  render(
    <MockedProvider>
      <MemoryRouter initialEntries={['/en']}>
        <PublicMetadataBoundary />
        <Link to="/vi/about">About</Link>
        <Routes>
          <Route path="*" element={<span>route</span>} />
        </Routes>
      </MemoryRouter>
    </MockedProvider>,
  );
  expect(document.title).toBe('Writing index — Test Site');
  await screen.findByRole('link', { name: 'About' });
  await userEvent.click(screen.getByRole('link', { name: 'About' }));
  expect(await screen.findByText('route')).toBeInTheDocument();
  await waitFor(() => expect(document.title).toBe('Về tôi — Test Site'));
  expect(document.documentElement.lang).toBe('vi');
  expect(document.head.querySelectorAll('[data-public-metadata]').length).toBe(
    12,
  );
});
