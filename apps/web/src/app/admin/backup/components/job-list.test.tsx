import { fireEvent, render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import JobList, { type JobListProps } from './job-list';
import type { BackupJob } from '@/domains/backup';

const job = (id: string, over: Partial<BackupJob> = {}): BackupJob => ({
  id,
  instance_id: 'local-1',
  instance_label: 'Local development instance',
  operation: 'backup_db_slice',
  status: 'queued',
  reason_code: null,
  synthetic: true,
  download_eligible: false,
  components: [],
  progress: null,
  correlation_id: `c-${id}`,
  created_at: '2026-11-22T10:00:00Z',
  started_at: null,
  finished_at: null,
  ...over,
});

const props = (over: Partial<JobListProps> = {}): JobListProps => ({
  jobs: [],
  phase: 'ready',
  polling: false,
  exhausted: false,
  selectedId: null,
  onSelect: () => {},
  ...over,
});

describe('JobList', () => {
  it('renders a labelled table with each job status as text', () => {
    render(<JobList {...props({ jobs: [job('job-a', { status: 'running' }), job('job-b', { status: 'incomplete' })] })} />);
    const table = screen.getByRole('table', { name: /backup jobs/i });
    expect(within(table).getByText('job-a')).toBeInTheDocument();
    expect(within(table).getByText('Running')).toBeInTheDocument();
    expect(within(table).getByText('Incomplete')).toBeInTheDocument();
    expect(within(table).getAllByRole('columnheader').length).toBeGreaterThanOrEqual(3);
  });

  it('ts12a_every_status_row_renders_without_any_download_control', () => {
    const statuses = ['queued', 'running', 'succeeded', 'failed', 'recovery-required', 'incomplete', 'unknown'] as const;
    const { container } = render(<JobList {...props({ jobs: statuses.map((status, i) => job(`job-${i}`, { status })) })} />);
    expect(screen.getAllByRole('row')).toHaveLength(statuses.length + 1);
    expect(screen.queryByRole('button', { name: /download/i })).not.toBeInTheDocument();
    expect(container.querySelector('a, [download]')).toBeNull();
  });

  // TS-37: keyboard operation
  it('ts37_a_job_is_selected_with_the_keyboard_and_marks_the_current_one', async () => {
    const onSelect = vi.fn();
    const user = userEvent.setup();
    const { rerender } = render(<JobList {...props({ jobs: [job('job-a'), job('job-b')], onSelect })} />);
    await user.tab();
    expect(screen.getByRole('button', { name: /job-a/ })).toHaveFocus();
    await user.tab();
    expect(screen.getByRole('button', { name: /job-b/ })).toHaveFocus();
    await user.keyboard('{Enter}');
    expect(onSelect).toHaveBeenCalledWith('job-b');
    rerender(<JobList {...props({ jobs: [job('job-a'), job('job-b')], onSelect, selectedId: 'job-b' })} />);
    expect(screen.getByRole('button', { name: /job-b/ })).toHaveAttribute('aria-current', 'true');
    expect(screen.getByRole('button', { name: /job-a/ })).not.toHaveAttribute('aria-current');
  });

  // TS-37: announcements
  it('ts37_announces_loading_empty_and_polling_states_through_a_polite_live_region', () => {
    const { rerender } = render(<JobList {...props({ phase: 'loading' })} />);
    expect(screen.getByRole('status')).toHaveTextContent(/loading backup jobs/i);

    rerender(<JobList {...props({ phase: 'ready' })} />);
    expect(screen.getByRole('status')).toHaveTextContent(/no backup jobs/i);

    rerender(<JobList {...props({ jobs: [job('job-a')], polling: true })} />);
    const live = screen.getByRole('status');
    expect(live).toHaveTextContent(/updating automatically/i);
    expect(live).toHaveAttribute('aria-live', 'polite');

    rerender(<JobList {...props({ jobs: [job('job-a', { status: 'incomplete' })], polling: false })} />);
    expect(screen.getByRole('status')).toHaveTextContent(/automatic updates have stopped|up to date/i);
  });

  it('shows an alert for a request failure that is still being retried, with fixed copy', () => {
    render(<JobList {...props({ jobs: [job('job-a')], phase: 'error', polling: true })} />);
    expect(screen.getByRole('alert')).toHaveTextContent(/unable to refresh backup jobs/i);
    expect(screen.getByRole('table', { name: /backup jobs/i })).toBeInTheDocument();
  });

  it('ts36_shows_an_error_banner_and_a_manual_refresh_when_unknown_status_polling_is_exhausted', () => {
    const onRefresh = vi.fn();
    render(<JobList {...props({ jobs: [job('job-a', { status: 'unknown' })], exhausted: true, onRefresh })} />);
    expect(screen.getByRole('alert')).toHaveTextContent(/status could not be determined/i);
    fireEvent.click(screen.getByRole('button', { name: /refresh/i }));
    expect(onRefresh).toHaveBeenCalledTimes(1);
  });

  // TS-18
  it('ts18_html_bearing_server_strings_render_escaped', () => {
    const canary = '<script>window.__canary=1</script>';
    const { container } = render(<JobList {...props({ jobs: [job(canary, { instance_label: canary })] })} />);
    expect(container.querySelector('script')).toBeNull();
    expect(container.innerHTML).toContain('&lt;script&gt;');
    expect(screen.getByRole('button', { name: new RegExp(canary.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')) })).toBeInTheDocument();
    expect((window as unknown as { __canary?: number }).__canary).toBeUndefined();
  });
});
