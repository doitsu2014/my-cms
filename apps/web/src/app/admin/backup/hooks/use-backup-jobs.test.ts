import { act, renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { BACKOFF_CEILING_MS, MAX_UNKNOWN_POLLS, POLL_INTERVAL_MS, useBackupJobs } from './use-backup-jobs';
import { BackupApiError, listBackupJobs, type BackupJob } from '@/domains/backup';

vi.mock('@/domains/backup', async () => {
  const actual = await vi.importActual<typeof import('@/domains/backup')>('@/domains/backup');
  return { ...actual, listBackupJobs: vi.fn() };
});

const listMock = vi.mocked(listBackupJobs);

const job = (status: BackupJob['status'], id = 'job-1'): BackupJob => ({
  id,
  instance_id: 'local-1',
  instance_label: 'Local development instance',
  operation: 'backup_db_slice',
  status,
  reason_code: null,
  synthetic: true,
  download_eligible: false,
  components: [],
  progress: null,
  correlation_id: 'c-1',
  created_at: '2026-11-22T10:00:00Z',
  started_at: null,
  finished_at: null,
});
const page = (...jobs: BackupJob[]) => ({ jobs, next_cursor: null });

const advance = (ms: number) => act(async () => { await vi.advanceTimersByTimeAsync(ms); });
const flush = () => advance(0);

describe('useBackupJobs (TS-36)', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    listMock.mockReset();
  });
  afterEach(() => {
    vi.useRealTimers();
  });

  it('ts36_declares_the_interface_contract_constants', () => {
    expect(POLL_INTERVAL_MS).toBe(2000);
    expect(BACKOFF_CEILING_MS).toBe(30000);
    expect(MAX_UNKNOWN_POLLS).toBe(10);
  });

  it('ts36_loads_once_on_mount_with_the_caller_token', async () => {
    listMock.mockResolvedValue(page(job('incomplete')));
    const { result } = renderHook(() => useBackupJobs('tok'));
    expect(result.current.phase).toBe('loading');
    await flush();
    expect(listMock).toHaveBeenCalledWith('tok');
    expect(result.current.phase).toBe('ready');
    expect(result.current.jobs).toHaveLength(1);
  });

  it('ts36_polls_every_2s_while_any_job_is_non_terminal', async () => {
    listMock.mockResolvedValue(page(job('queued'), job('incomplete', 'job-2')));
    const { result } = renderHook(() => useBackupJobs('tok'));
    await flush();
    expect(listMock).toHaveBeenCalledTimes(1);
    expect(result.current.polling).toBe(true);
    await advance(1999);
    expect(listMock).toHaveBeenCalledTimes(1);
    await advance(1);
    expect(listMock).toHaveBeenCalledTimes(2);
    await advance(2000);
    expect(listMock).toHaveBeenCalledTimes(3);
  });

  it('ts36_stops_polling_when_every_job_is_terminal', async () => {
    listMock
      .mockResolvedValueOnce(page(job('running')))
      .mockResolvedValueOnce(page(job('incomplete')))
      .mockResolvedValue(page(job('incomplete')));
    const { result } = renderHook(() => useBackupJobs('tok'));
    await flush();
    await advance(2000);
    expect(listMock).toHaveBeenCalledTimes(2);
    expect(result.current.jobs[0].status).toBe('incomplete');
    expect(result.current.polling).toBe(false);
    await advance(60000);
    expect(listMock).toHaveBeenCalledTimes(2);
  });

  it('ts36_does_not_poll_at_all_when_the_first_list_is_all_terminal_or_empty', async () => {
    listMock.mockResolvedValue(page());
    renderHook(() => useBackupJobs('tok'));
    await flush();
    await advance(30000);
    expect(listMock).toHaveBeenCalledTimes(1);
  });

  it('ts36_backs_off_by_doubling_from_2s_to_a_30s_ceiling_on_request_errors', async () => {
    listMock.mockRejectedValue(new BackupApiError(503, 'STORE_UNAVAILABLE'));
    const { result } = renderHook(() => useBackupJobs('tok'));
    await flush();
    expect(listMock).toHaveBeenCalledTimes(1);
    expect(result.current.phase).toBe('error');

    let calls = 1;
    for (const wait of [2000, 4000, 8000, 16000, 30000, 30000]) {
      await advance(wait - 1);
      expect(listMock).toHaveBeenCalledTimes(calls);
      await advance(1);
      calls += 1;
      expect(listMock).toHaveBeenCalledTimes(calls);
    }
  });

  it('ts36_resets_the_backoff_after_a_success', async () => {
    listMock
      .mockRejectedValueOnce(new BackupApiError(503))
      .mockRejectedValueOnce(new BackupApiError(503))
      .mockResolvedValueOnce(page(job('running')))
      .mockRejectedValueOnce(new BackupApiError(503))
      .mockResolvedValue(page(job('incomplete')));
    const { result } = renderHook(() => useBackupJobs('tok'));
    await flush(); // fail, next in 2s
    await advance(2000); // fail, next in 4s
    await advance(4000); // success (running), next in 2s
    expect(result.current.phase).toBe('ready');
    expect(listMock).toHaveBeenCalledTimes(3);
    await advance(2000); // fail again, backoff restarts at 2s (not 8s)
    expect(listMock).toHaveBeenCalledTimes(4);
    expect(result.current.phase).toBe('error');
    await advance(2000);
    expect(listMock).toHaveBeenCalledTimes(5);
    expect(result.current.phase).toBe('ready');
  });

  it('ts36_keeps_the_last_jobs_while_a_poll_fails', async () => {
    listMock.mockResolvedValueOnce(page(job('running'))).mockRejectedValue(new BackupApiError(500));
    const { result } = renderHook(() => useBackupJobs('tok'));
    await flush();
    await advance(2000);
    expect(result.current.phase).toBe('error');
    expect(result.current.jobs).toHaveLength(1);
  });

  it('ts36_treats_unknown_as_non_terminal_for_at_most_10_polls_then_stops_with_exhausted', async () => {
    listMock.mockResolvedValue(page(job('unknown')));
    const { result } = renderHook(() => useBackupJobs('tok'));
    await flush();
    expect(result.current.exhausted).toBe(false);
    await advance(2000 * 20);
    expect(listMock).toHaveBeenCalledTimes(MAX_UNKNOWN_POLLS);
    expect(result.current.exhausted).toBe(true);
    expect(result.current.polling).toBe(false);
    await advance(60000);
    expect(listMock).toHaveBeenCalledTimes(MAX_UNKNOWN_POLLS);
  });

  it('ts36_an_unknown_status_that_resolves_to_terminal_stops_normally_without_exhaustion', async () => {
    listMock
      .mockResolvedValueOnce(page(job('unknown')))
      .mockResolvedValueOnce(page(job('unknown')))
      .mockResolvedValue(page(job('incomplete')));
    const { result } = renderHook(() => useBackupJobs('tok'));
    await flush();
    await advance(4000);
    expect(result.current.exhausted).toBe(false);
    expect(result.current.polling).toBe(false);
    await advance(30000);
    expect(listMock).toHaveBeenCalledTimes(3);
  });

  it('ts36_cleans_up_on_unmount_so_no_timer_or_request_survives', async () => {
    listMock.mockResolvedValue(page(job('running')));
    const { unmount } = renderHook(() => useBackupJobs('tok'));
    await flush();
    expect(listMock).toHaveBeenCalledTimes(1);
    expect(vi.getTimerCount()).toBeGreaterThan(0);
    unmount();
    expect(vi.getTimerCount()).toBe(0);
    await advance(60000);
    expect(listMock).toHaveBeenCalledTimes(1);
  });

  it('ts36_an_in_flight_response_after_unmount_is_ignored_and_schedules_nothing', async () => {
    let resolve!: (value: ReturnType<typeof page>) => void;
    listMock.mockReturnValue(new Promise((r) => { resolve = r; }));
    const { unmount } = renderHook(() => useBackupJobs('tok'));
    unmount();
    await act(async () => { resolve(page(job('running'))); });
    expect(vi.getTimerCount()).toBe(0);
    await advance(60000);
    expect(listMock).toHaveBeenCalledTimes(1);
  });

  it('ts36_refresh_reloads_immediately_and_restarts_polling_after_a_stop', async () => {
    listMock.mockResolvedValueOnce(page(job('incomplete'))).mockResolvedValue(page(job('queued', 'job-2'), job('incomplete')));
    const { result } = renderHook(() => useBackupJobs('tok'));
    await flush();
    expect(result.current.polling).toBe(false);
    act(() => result.current.refresh());
    await flush();
    expect(listMock).toHaveBeenCalledTimes(2);
    expect(result.current.polling).toBe(true);
    await advance(2000);
    expect(listMock).toHaveBeenCalledTimes(3);
  });

  it('ts36_a_token_change_restarts_the_load_with_the_new_token', async () => {
    listMock.mockResolvedValue(page(job('incomplete')));
    const { rerender } = renderHook(({ token }) => useBackupJobs(token), { initialProps: { token: 'a' as string | null } });
    await flush();
    rerender({ token: 'b' });
    await flush();
    expect(listMock).toHaveBeenLastCalledWith('b');
    expect(listMock).toHaveBeenCalledTimes(2);
  });
});
