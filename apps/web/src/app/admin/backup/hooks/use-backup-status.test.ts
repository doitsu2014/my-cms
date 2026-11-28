import { act, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useBackupStatus } from './use-backup-status';
import { BackupApiError, getBackupStatus, type BackupStatus } from '@/domains/backup';

vi.mock('@/domains/backup', async () => {
  const actual = await vi.importActual<typeof import('@/domains/backup')>('@/domains/backup');
  return { ...actual, getBackupStatus: vi.fn() };
});

const statusMock = vi.mocked(getBackupStatus);

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

describe('useBackupStatus (F-49)', () => {
  beforeEach(() => {
    statusMock.mockReset();
  });

  it('loads the IF-11 status once on mount with the caller token', async () => {
    statusMock.mockResolvedValue(ready);
    const { result } = renderHook(() => useBackupStatus('tok'));
    expect(result.current.phase).toBe('loading');
    await act(async () => {});
    expect(statusMock).toHaveBeenCalledWith('tok');
    expect(result.current.phase).toBe('ready');
    expect(result.current.status).toEqual(ready);
  });

  it('reports an error phase with no status when the request fails', async () => {
    statusMock.mockRejectedValue(new BackupApiError(403, 'Forbidden', 'ROLE_REQUIRED'));
    const { result } = renderHook(() => useBackupStatus('tok'));
    await act(async () => {});
    expect(result.current.phase).toBe('error');
    expect(result.current.status).toBeNull();
  });

  it('refresh reloads the status', async () => {
    statusMock.mockResolvedValueOnce(ready).mockResolvedValueOnce({ ...ready, state: 'degraded' });
    const { result } = renderHook(() => useBackupStatus('tok'));
    await act(async () => {});
    act(() => result.current.refresh());
    await act(async () => {});
    expect(statusMock).toHaveBeenCalledTimes(2);
    expect(result.current.status?.state).toBe('degraded');
  });

  it('ignores a response that arrives after unmount', async () => {
    let resolve!: (value: BackupStatus) => void;
    statusMock.mockReturnValue(new Promise((r) => { resolve = r; }));
    const { result, unmount } = renderHook(() => useBackupStatus('tok'));
    unmount();
    await act(async () => { resolve(ready); });
    expect(result.current.status).toBeNull();
  });
});
