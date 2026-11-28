import { useRef, useState } from 'react';
import { createBackupJob, type BackupJob } from '@/domains/backup';

export interface JobCreatePanelProps {
  token: string | null;
  instanceId: string | null;
  onCreated?: (job: BackupJob, replayed: boolean) => void;
}

/**
 * F-44: creates a backup job for the selected instance. Every submit sends a fresh `crypto.randomUUID()`
 * idempotency key. A ref guard (not just the disabled state) stops a second request while one is in flight,
 * because two clicks can land before React re-renders. The failure copy is fixed text: server strings are
 * never shown. Client checks are UX only; the server enforces authorization and validation.
 */
export default function JobCreatePanel({ token, instanceId, onCreated }: JobCreatePanelProps) {
  const inFlight = useRef(false);
  const [submitting, setSubmitting] = useState(false);
  const [failed, setFailed] = useState(false);

  const submit = async () => {
    if (!instanceId || inFlight.current) return;
    inFlight.current = true;
    setSubmitting(true);
    setFailed(false);
    try {
      const { job, replayed } = await createBackupJob(token, instanceId, crypto.randomUUID());
      onCreated?.(job, replayed);
    } catch {
      setFailed(true);
    } finally {
      inFlight.current = false;
      setSubmitting(false);
    }
  };

  return (
    <div className="space-y-3">
      {failed && (
        <div role="alert" className="alert alert-error">
          <span>Unable to create the backup job. Try again.</span>
        </div>
      )}
      <button
        type="button"
        className="btn btn-primary"
        disabled={!instanceId || submitting}
        onClick={() => void submit()}
      >
        {submitting && <span className="loading loading-spinner loading-sm" aria-hidden="true" />}
        Create backup
      </button>
    </div>
  );
}
