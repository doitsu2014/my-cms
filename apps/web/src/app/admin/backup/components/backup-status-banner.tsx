import type { BackupDisabledReason, BackupStatus } from '@/domains/backup';

export interface BackupStatusBannerProps {
  status: BackupStatus | null;
  phase: 'loading' | 'ready' | 'error';
}

function disabledReasonText(reason: BackupDisabledReason | null): string {
  switch (reason) {
    case 'NOT_CONFIGURED':
      return 'Backup is not configured on this server.';
    case 'NON_LOOPBACK_BIND':
      return 'The API is not bound to a loopback address, so backup stays off.';
    case 'INVALID_CONFIG':
      return 'The backup configuration is invalid.';
    case null:
      return 'No reason was reported.';
    default: {
      const unreachable: never = reason;
      return unreachable;
    }
  }
}

function stateText(state: BackupStatus['state']): string {
  switch (state) {
    case 'ready':
      return 'Backup is ready. Jobs are synthetic and database-only.';
    case 'not_ready':
      return 'Backup is not ready yet. The first reconciliation has not completed.';
    case 'degraded':
      return 'Backup is degraded.';
    case 'disabled':
      return 'Backup is disabled.';
    default: {
      const unreachable: never = state;
      return unreachable;
    }
  }
}

const plural = (count: number, singular: string, pluralForm: string) => `${count} ${count === 1 ? singular : pluralForm}`;

/**
 * F-49: renders the IF-11 status. IF-11 answers 200 even when backup is disabled, so a disabled state and its
 * reason are shown from the body. Escalation names the project owner (owner decision Q-05). Only the closed,
 * schema-validated vocabulary is mapped to copy; the owner string is the one server string shown, as React text.
 */
export default function BackupStatusBanner({ status, phase }: BackupStatusBannerProps) {
  if (!status) {
    if (phase === 'error') {
      return (
        <div role="alert" className="alert alert-warning">
          <span>Backup status is unavailable.</span>
        </div>
      );
    }
    return (
      <div role="status" className="alert">
        <span>Checking backup status.</span>
      </div>
    );
  }

  const exhausted = status.cleanup?.exhausted ?? 0;
  const incidents = status.cleanup?.ownership_incidents ?? 0;
  const escalate = status.escalation.visible || status.state === 'disabled';
  const urgent = escalate || status.state === 'degraded';
  const ownerLabel = status.escalation.owner.replace(/_/g, ' ');

  return (
    <div role={urgent ? 'alert' : 'status'} className={`alert ${urgent ? 'alert-warning' : ''}`}>
      <div className="space-y-1">
        <p className="font-medium">{stateText(status.state)}</p>
        {status.state === 'disabled' && <p>{disabledReasonText(status.disabled_reason)}</p>}
        {!status.db_healthy && <p>The backup database is unavailable.</p>}
        {exhausted > 0 && (
          <p>{plural(exhausted, 'cleanup item', 'cleanup items')} could not be removed after repeated attempts.</p>
        )}
        {incidents > 0 && <p>{plural(incidents, 'ownership incident needs', 'ownership incidents need')} review.</p>}
        {escalate && <p>Escalate to the {ownerLabel}.</p>}
      </div>
    </div>
  );
}
