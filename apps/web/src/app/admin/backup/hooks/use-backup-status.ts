import { useCallback, useEffect, useState } from 'react';
import { getBackupStatus, type BackupStatus } from '@/domains/backup';

export interface UseBackupStatus {
  status: BackupStatus | null;
  phase: 'loading' | 'ready' | 'error';
  refresh: () => void;
}

/**
 * F-49: loads the IF-11 status for the banner. IF-11 answers 200 for an administrator even when backup is
 * disabled, so a failure here is a request or authorization problem and shows as `error`. The status is
 * loaded on mount and on `refresh`; a response that arrives after unmount or a token change is ignored.
 */
export function useBackupStatus(token: string | null): UseBackupStatus {
  const [status, setStatus] = useState<BackupStatus | null>(null);
  const [phase, setPhase] = useState<UseBackupStatus['phase']>('loading');
  const [generation, setGeneration] = useState(0);

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      try {
        const loaded = await getBackupStatus(token);
        if (cancelled) return;
        setStatus(loaded);
        setPhase('ready');
      } catch {
        if (cancelled) return;
        setStatus(null);
        setPhase('error');
      }
    };
    void load();
    return () => {
      cancelled = true;
    };
  }, [token, generation]);

  const refresh = useCallback(() => setGeneration((value) => value + 1), []);

  return { status, phase, refresh };
}
