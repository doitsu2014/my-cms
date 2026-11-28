import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import AdminBackupPage from './page';
import {
  createBackupJob,
  getBackupStatus,
  listBackupInstances,
  listBackupJobs,
  type BackupInstance,
  type BackupJob,
  type BackupStatus,
} from '@/domains/backup';

vi.mock('@/auth/AuthContext', () => ({ useAuth: () => ({ token: 'tok', userInfo: { name: 'Admin', email: 'admin@example.com' } }) }));
vi.mock('@/domains/backup', async () => {
  const actual = await vi.importActual<typeof import('@/domains/backup')>('@/domains/backup');
  return {
    ...actual,
    listBackupInstances: vi.fn(),
    createBackupJob: vi.fn(),
    listBackupJobs: vi.fn(),
    getBackupStatus: vi.fn(),
  };
});

const instancesMock = vi.mocked(listBackupInstances);
const createMock = vi.mocked(createBackupJob);
const jobsMock = vi.mocked(listBackupJobs);
const statusMock = vi.mocked(getBackupStatus);

const instance: BackupInstance = {
  id: 'local-1',
  label: 'Local development instance',
  environment: 'local',
  operations: ['backup_db_slice'],
  capabilities: { create: true, upload: false, preflight: true, restore: false, download: false },
};
const job = (id: string, over: Partial<BackupJob> = {}): BackupJob => ({
  id,
  instance_id: 'local-1',
  instance_label: 'Local development instance',
  operation: 'backup_db_slice',
  status: 'incomplete',
  reason_code: 'REQUIRED_COMPONENTS_UNAVAILABLE',
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
const ready: BackupStatus = {
  state: 'ready',
  disabled_reason: null,
  db_healthy: true,
  admissions_open: true,
  synthetic_only: true,
  reconciliation: null,
  cleanup: null,
  escalation: { owner: 'project_owner', visible: false },
};

const renderPage = () => render(<MemoryRouter><AdminBackupPage /></MemoryRouter>);

describe('AdminBackupPage status wiring (WU-14)', () => {
  beforeEach(() => {
    instancesMock.mockReset().mockResolvedValue({ synthetic: true, instances: [instance] });
    createMock.mockReset();
    jobsMock.mockReset().mockResolvedValue({ jobs: [job('job-a'), job('job-b', { status: 'failed' })], next_cursor: null });
    statusMock.mockReset().mockResolvedValue(ready);
  });

  it('renders the status banner, the job list and the synthetic preview panel', async () => {
    renderPage();
    expect(await screen.findByRole('table', { name: /backup jobs/i })).toBeInTheDocument();
    expect(await screen.findByText(/backup is ready/i)).toBeInTheDocument();
    expect(statusMock).toHaveBeenCalledWith('tok');
    expect(jobsMock).toHaveBeenCalledWith('tok');
    expect(screen.getByRole('heading', { name: /synthetic restore preview/i })).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /download/i })).not.toBeInTheDocument();
  });

  it('shows the escalation banner with the project owner when IF-11 says so', async () => {
    statusMock.mockResolvedValue({
      ...ready,
      state: 'degraded',
      cleanup: { pending: 0, exhausted: 1, orphan_discovered: 0, ownership_incidents: 0 },
      escalation: { owner: 'project_owner', visible: true },
    });
    renderPage();
    expect(await screen.findByText(/escalate to the project owner/i)).toBeInTheDocument();
  });

  it('selecting a job shows its detail with status text and no download control', async () => {
    renderPage();
    const table = await screen.findByRole('table', { name: /backup jobs/i });
    fireEvent.click(within(table).getByRole('button', { name: /job-b/ }));
    const detail = await screen.findByRole('region', { name: /backup job details/i });
    expect(within(detail).getByText('Failed')).toBeInTheDocument();
    expect(within(detail).getByText('c-job-b')).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /download/i })).not.toBeInTheDocument();
  });

  it('refreshes the job list and status after a job is created', async () => {
    const created = job('job-new', { status: 'queued', reason_code: null });
    createMock.mockResolvedValue({ job: created, replayed: false });
    renderPage();
    await screen.findByRole('table', { name: /backup jobs/i });
    const before = jobsMock.mock.calls.length;
    const statusBefore = statusMock.mock.calls.length;
    jobsMock.mockResolvedValue({ jobs: [created, job('job-a')], next_cursor: null });
    await waitFor(() => expect(screen.getByRole('button', { name: /create backup/i })).toBeEnabled());
    fireEvent.click(screen.getByRole('button', { name: /create backup/i }));
    await waitFor(() => expect(jobsMock.mock.calls.length).toBeGreaterThan(before));
    expect(statusMock.mock.calls.length).toBeGreaterThan(statusBefore);
    expect(await screen.findByRole('button', { name: /job-new/ })).toBeInTheDocument();
  });

  it('keeps the page usable when the jobs and status requests fail', async () => {
    jobsMock.mockRejectedValue(new Error('boom'));
    statusMock.mockRejectedValue(new Error('boom'));
    renderPage();
    expect(await screen.findByText(/backup status is unavailable/i)).toBeInTheDocument();
    expect(await screen.findByText(/unable to refresh backup jobs/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /create backup/i })).toBeInTheDocument();
  });
});
