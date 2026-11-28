import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import JobCreatePanel from './job-create-panel';
import { BackupApiError, createBackupJob, type BackupJob } from '@/domains/backup';

vi.mock('@/domains/backup', async () => {
  const actual = await vi.importActual<typeof import('@/domains/backup')>('@/domains/backup');
  return { ...actual, createBackupJob: vi.fn() };
});

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

const createMock = vi.mocked(createBackupJob);
const submit = () => screen.getByRole('button', { name: /create backup/i });

describe('JobCreatePanel', () => {
  beforeEach(() => {
    createMock.mockReset();
  });
  afterEach(() => {
    vi.restoreAllMocks();
  });

  // TS-52 panel part: fresh key per submit
  it('ts52_sends_a_fresh_crypto_random_uuid_idempotency_key_for_every_submit', async () => {
    const uuid = vi.spyOn(globalThis.crypto, 'randomUUID');
    uuid
      .mockReturnValueOnce('11111111-1111-4111-8111-111111111111')
      .mockReturnValueOnce('22222222-2222-4222-8222-222222222222');
    createMock.mockResolvedValue({ job, replayed: false });
    render(<JobCreatePanel token="tok" instanceId="inst-1" />);

    fireEvent.click(submit());
    await waitFor(() => expect(createMock).toHaveBeenCalledTimes(1));
    await waitFor(() => expect(submit()).toBeEnabled());
    fireEvent.click(submit());
    await waitFor(() => expect(createMock).toHaveBeenCalledTimes(2));

    expect(createMock).toHaveBeenNthCalledWith(1, 'tok', 'inst-1', '11111111-1111-4111-8111-111111111111');
    expect(createMock).toHaveBeenNthCalledWith(2, 'tok', 'inst-1', '22222222-2222-4222-8222-222222222222');
  });

  // TS-52 panel part: double-submit guard
  it('ts52_double_submit_guard_sends_one_request_while_one_is_in_flight', async () => {
    let resolve!: (value: { job: BackupJob; replayed: boolean }) => void;
    createMock.mockReturnValue(new Promise((r) => { resolve = r; }));
    render(<JobCreatePanel token="tok" instanceId="inst-1" />);

    fireEvent.click(submit());
    fireEvent.click(submit());
    fireEvent.click(submit());
    expect(createMock).toHaveBeenCalledTimes(1);
    expect(submit()).toBeDisabled();

    await act(async () => { resolve({ job, replayed: false }); });
    await waitFor(() => expect(submit()).toBeEnabled());
    expect(createMock).toHaveBeenCalledTimes(1);
  });

  it('ts52_submit_without_a_selected_instance_sends_nothing', () => {
    render(<JobCreatePanel token="tok" instanceId={null} />);
    expect(submit()).toBeDisabled();
    fireEvent.click(submit());
    expect(createMock).not.toHaveBeenCalled();
  });

  it('reports the created job to the caller', async () => {
    createMock.mockResolvedValue({ job, replayed: true });
    const onCreated = vi.fn();
    render(<JobCreatePanel token="tok" instanceId="inst-1" onCreated={onCreated} />);
    fireEvent.click(submit());
    await waitFor(() => expect(onCreated).toHaveBeenCalledWith(job, true));
  });

  it('shows fixed failure copy, never server strings, and allows a retry with a new key', async () => {
    const uuid = vi.spyOn(globalThis.crypto, 'randomUUID');
    uuid
      .mockReturnValueOnce('aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa')
      .mockReturnValueOnce('bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb');
    createMock.mockRejectedValueOnce(new BackupApiError(503, 'BACKUP_DISABLED', 'NOT_CONFIGURED', 'corr-9'));
    createMock.mockResolvedValueOnce({ job, replayed: false });
    render(<JobCreatePanel token="tok" instanceId="inst-1" />);

    fireEvent.click(submit());
    expect(await screen.findByRole('alert')).toHaveTextContent(/unable to create the backup job/i);
    await waitFor(() => expect(submit()).toBeEnabled());

    fireEvent.click(submit());
    await waitFor(() => expect(createMock).toHaveBeenCalledTimes(2));
    expect(createMock.mock.calls[0][2]).toBe('aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa');
    expect(createMock.mock.calls[1][2]).toBe('bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb');
    await waitFor(() => expect(screen.queryByRole('alert')).not.toBeInTheDocument());
  });
});
