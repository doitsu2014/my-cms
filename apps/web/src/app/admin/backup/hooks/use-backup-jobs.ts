import { useCallback, useEffect, useState } from 'react';
import { isTerminalBackupJobStatus, listBackupJobs, type BackupJob } from '@/domains/backup';

/** Polling constants from interfaces-and-contracts 3.2 (non-owner technical guards). */
export const POLL_INTERVAL_MS = 2000;
export const BACKOFF_CEILING_MS = 30000;
export const MAX_UNKNOWN_POLLS = 10;

export interface UseBackupJobs {
  jobs: BackupJob[];
  phase: 'loading' | 'ready' | 'error';
  /** True while another poll is scheduled because some job is not terminal. */
  polling: boolean;
  /** True when polling stopped because a status stayed `unknown` for MAX_UNKNOWN_POLLS polls. */
  exhausted: boolean;
  /** Reload now and restart polling (also resets the unknown-status budget and the backoff). */
  refresh: () => void;
}

/**
 * F-46: loads the job list and polls while any job is non-terminal (T-13).
 *
 * - One request at a time: the next poll is scheduled only after the previous one settles.
 * - Every 2 s while any job is non-terminal; stops when all are terminal (`incomplete`, `failed`, and the
 *   retained `succeeded` / `recovery-required`).
 * - A failed request keeps the last jobs, sets `phase: 'error'` and retries with a delay that doubles from 2 s
 *   to a 30 s ceiling; one success resets it.
 * - `unknown` counts as non-terminal, but at most MAX_UNKNOWN_POLLS polls in a row return one before polling
 *   stops with `exhausted`, so an unrecognised status never polls forever.
 * - Unmount, a token change or refresh cancel the pending timer and ignore any in-flight response.
 */
export function useBackupJobs(token: string | null): UseBackupJobs {
  const [jobs, setJobs] = useState<BackupJob[]>([]);
  const [phase, setPhase] = useState<UseBackupJobs['phase']>('loading');
  const [polling, setPolling] = useState(false);
  const [exhausted, setExhausted] = useState(false);
  const [generation, setGeneration] = useState(0);

  useEffect(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let backoff = POLL_INTERVAL_MS;
    let unknownPolls = 0;

    const schedule = (delay: number) => {
      timer = setTimeout(() => void poll(), delay);
    };

    const poll = async () => {
      try {
        const { jobs: listed } = await listBackupJobs(token);
        if (cancelled) return;
        backoff = POLL_INTERVAL_MS;
        setJobs(listed);
        setPhase('ready');

        unknownPolls = listed.some((job) => job.status === 'unknown') ? unknownPolls + 1 : 0;
        if (unknownPolls >= MAX_UNKNOWN_POLLS) {
          setExhausted(true);
          setPolling(false);
          return;
        }
        setExhausted(false);
        const active = listed.some((job) => !isTerminalBackupJobStatus(job.status));
        setPolling(active);
        if (active) schedule(POLL_INTERVAL_MS);
      } catch {
        if (cancelled) return;
        setPhase('error');
        setPolling(true);
        schedule(backoff);
        backoff = Math.min(backoff * 2, BACKOFF_CEILING_MS);
      }
    };

    void poll();
    return () => {
      cancelled = true;
      if (timer !== undefined) clearTimeout(timer);
    };
  }, [token, generation]);

  const refresh = useCallback(() => setGeneration((value) => value + 1), []);

  return { jobs, phase, polling, exhausted, refresh };
}
