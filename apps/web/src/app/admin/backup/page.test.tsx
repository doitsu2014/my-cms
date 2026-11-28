import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import AdminBackupPage from './page';
import { BackupApiError, createBackupJob, listBackupInstances, type BackupInstance, type BackupJob } from '@/domains/backup';

vi.mock('@/auth/AuthContext', () => ({ useAuth: () => ({ token: 'tok', userInfo: { name: 'Admin', email: 'admin@example.com' } }) }));
vi.mock('@/domains/backup', async () => {
  const actual = await vi.importActual<typeof import('@/domains/backup')>('@/domains/backup');
  // WU-14 test-support: the page now also loads the job list and IF-11 status through these two calls.
  return {
    ...actual,
    listBackupInstances: vi.fn(),
    createBackupJob: vi.fn(),
    listBackupJobs: vi.fn(async () => ({ jobs: [], next_cursor: null })),
    getBackupStatus: vi.fn(async () => ({
      state: 'ready',
      disabled_reason: null,
      db_healthy: true,
      admissions_open: true,
      synthetic_only: true,
      reconciliation: null,
      cleanup: null,
      escalation: { owner: 'project_owner', visible: false },
    })),
  };
});

const listMock = vi.mocked(listBackupInstances);
const createMock = vi.mocked(createBackupJob);

const instance: BackupInstance = {
  id: 'inst-1',
  label: 'Local synthetic instance',
  environment: 'development',
  operations: ['create'],
  capabilities: { create: true, upload: false, preflight: false, restore: false, download: false },
};
const job: BackupJob = {
  id: 'job-1',
  instance_id: 'inst-1',
  instance_label: 'Local synthetic instance',
  operation: 'create',
  status: 'queued',
  reason_code: null,
  synthetic: true,
  download_eligible: false,
  components: [],
  progress: null,
  correlation_id: 'corr-1',
  created_at: '2026-11-28T00:00:00Z',
  started_at: null,
  finished_at: null,
};

const renderPage = () => render(<MemoryRouter><AdminBackupPage /></MemoryRouter>);
const submit = () => screen.getByRole('button', { name: /create backup/i });

describe('AdminBackupPage', () => {
  beforeEach(() => {
    listMock.mockReset();
    createMock.mockReset();
  });

  it('lists instances with the caller token and renders label and environment', async () => {
    listMock.mockResolvedValue({ synthetic: true, instances: [instance] });
    renderPage();
    expect(await screen.findByText('Local synthetic instance')).toBeInTheDocument();
    expect(screen.getByText('development')).toBeInTheDocument();
    expect(listMock).toHaveBeenCalledWith('tok');
    expect(screen.getByRole('heading', { level: 1, name: /backup/i })).toBeInTheDocument();
  });

  // TS-38
  it('ts38_empty_instance_list_disables_submission', async () => {
    listMock.mockResolvedValue({ synthetic: true, instances: [] });
    renderPage();
    expect(await screen.findByText(/no backup instance is available/i)).toBeInTheDocument();
    expect(submit()).toBeDisabled();
    fireEvent.click(submit());
    expect(createMock).not.toHaveBeenCalled();
  });

  it('ts38_unavailable_instance_list_disables_submission_and_can_be_retried', async () => {
    listMock.mockRejectedValueOnce(new BackupApiError(503, 'BACKUP_DISABLED', 'NOT_CONFIGURED'));
    renderPage();
    expect(await screen.findByRole('alert')).toHaveTextContent(/unable to load/i);
    expect(submit()).toBeDisabled();

    listMock.mockResolvedValueOnce({ synthetic: true, instances: [instance] });
    fireEvent.click(screen.getByRole('button', { name: /retry/i }));
    await waitFor(() => expect(submit()).toBeEnabled());
    expect(listMock).toHaveBeenCalledTimes(2);
  });

  it('ts38_submission_is_disabled_while_the_list_is_loading', () => {
    listMock.mockReturnValue(new Promise(() => {}));
    renderPage();
    expect(submit()).toBeDisabled();
  });

  it('selects the single listed instance and creates a job for it with a fresh key', async () => {
    listMock.mockResolvedValue({ synthetic: true, instances: [instance] });
    createMock.mockResolvedValue({ job, replayed: false });
    renderPage();
    await waitFor(() => expect(submit()).toBeEnabled());
    fireEvent.click(submit());
    await waitFor(() => expect(createMock).toHaveBeenCalledTimes(1));
    const [token, instanceId, key] = createMock.mock.calls[0];
    expect(token).toBe('tok');
    expect(instanceId).toBe('inst-1');
    expect(key).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/);
    expect(await screen.findByText(/job-1/)).toBeInTheDocument();
  });
});
