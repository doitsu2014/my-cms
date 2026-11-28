import { render, screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import JobDetail from './job-detail';
import { backupJobSchema, KNOWN_BACKUP_JOB_STATUSES, type BackupJob } from '@/domains/backup';

const CANARY = '<img src=x onerror="window.__canary=1">';

const baseJob: BackupJob = {
  id: '0b6f0f0e-6a9c-4f51-8f3c-3d2a1c9e7a10',
  instance_id: 'local-1',
  instance_label: 'Local development instance',
  operation: 'backup_db_slice',
  status: 'queued',
  reason_code: null,
  synthetic: true,
  download_eligible: false,
  components: [],
  progress: null,
  correlation_id: 'c-7f3a9e12',
  created_at: '2026-11-22T10:00:00Z',
  started_at: null,
  finished_at: null,
};

const incompleteJob: BackupJob = {
  ...baseJob,
  status: 'incomplete',
  reason_code: 'REQUIRED_COMPONENTS_UNAVAILABLE',
  components: [
    { name: 'database', state: 'synthetic_captured', reason_code: 'DB_SYNTHETIC_ONLY' },
    { name: 'auth', state: 'unavailable', reason_code: 'AUTH_NOT_CAPTURED' },
    { name: 'storage', state: 'unavailable', reason_code: 'STORAGE_NOT_CAPTURED' },
  ],
  progress: { completed: 1, total: 1 },
  started_at: '2026-11-22T10:00:01Z',
  finished_at: '2026-11-22T10:00:02Z',
};

describe('JobDetail', () => {
  // TS-12a: every status passes the zod schema, renders, and download is always denied
  it.each(KNOWN_BACKUP_JOB_STATUSES)('ts12a_status_%s_parses_renders_and_offers_no_download', (status) => {
    const parsed = backupJobSchema.parse({ ...baseJob, status });
    const { container } = render(<JobDetail job={parsed} />);
    expect(screen.getByRole('heading', { name: /backup job details/i })).toBeInTheDocument();
    expect(screen.getByText(baseJob.id)).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /download/i })).not.toBeInTheDocument();
    expect(screen.queryByRole('link')).not.toBeInTheDocument();
    expect(container.querySelector('a, [download]')).toBeNull();
    expect(screen.getByText(/download is not available/i)).toBeInTheDocument();
  });

  it('ts12a_an_unrecognised_status_renders_the_safe_fallback_with_no_download', () => {
    const parsed = backupJobSchema.parse({ ...baseJob, status: 'archived' });
    expect(parsed.status).toBe('unknown');
    render(<JobDetail job={parsed} />);
    expect(screen.getByText('Unknown status')).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /download/i })).not.toBeInTheDocument();
  });

  it('shows the incomplete outcome, reason, components and measured progress as text', () => {
    render(<JobDetail job={incompleteJob} />);
    expect(screen.getByText('Incomplete')).toBeInTheDocument();
    expect(screen.getByText(/synthetic, database-only, not restorable/i)).toBeInTheDocument();
    expect(screen.getByText('REQUIRED_COMPONENTS_UNAVAILABLE')).toBeInTheDocument();
    const components = screen.getByRole('list', { name: /components/i });
    expect(within(components).getAllByRole('listitem')).toHaveLength(3);
    expect(within(components).getByText(/database/i)).toBeInTheDocument();
    expect(within(components).getByText('AUTH_NOT_CAPTURED')).toBeInTheDocument();
    expect(screen.getByText(/1 of 1/)).toBeInTheDocument();
    expect(screen.getByText('c-7f3a9e12')).toBeInTheDocument();
    expect(screen.getByText('Local development instance')).toBeInTheDocument();
  });

  it('says progress is not measured when the server reports none, and shows no components message', () => {
    render(<JobDetail job={baseJob} />);
    expect(screen.getByText(/not measured/i)).toBeInTheDocument();
    expect(screen.getByText(/no component results recorded/i)).toBeInTheDocument();
  });

  // TS-18: server strings are React text only
  it('ts18_html_bearing_server_strings_render_escaped_and_never_as_markup', () => {
    const job: BackupJob = {
      ...incompleteJob,
      instance_label: CANARY,
      reason_code: CANARY,
      correlation_id: CANARY,
      operation: CANARY,
      components: [{ name: 'database', state: 'unavailable', reason_code: CANARY }],
    };
    const { container } = render(<JobDetail job={job} />);
    expect(container.querySelector('img')).toBeNull();
    expect(container.querySelector('[onerror]')).toBeNull();
    expect(container.innerHTML).not.toContain('<img');
    expect(container.innerHTML).toContain('&lt;img');
    expect(screen.getAllByText(CANARY).length).toBeGreaterThanOrEqual(3);
    expect((window as unknown as { __canary?: number }).__canary).toBeUndefined();
  });
});
