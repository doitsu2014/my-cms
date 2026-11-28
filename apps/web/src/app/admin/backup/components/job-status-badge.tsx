import { AlertTriangle, CheckCircle2, CircleHelp, Clock, Loader, MinusCircle, XCircle, type LucideIcon } from 'lucide-react';
import type { BackupJobStatusOrUnknown } from '@/domains/backup';

export interface JobStatusBadgeProps {
  status: BackupJobStatusOrUnknown;
  /** Adds the plain-language outcome note for `incomplete` (F-45). */
  detailed?: boolean;
}

interface StatusView {
  label: string;
  icon: LucideIcon;
  tone: string;
  note?: string;
}

/**
 * Exhaustive over every documented status plus `unknown` (TS-12a). Adding a status to the type breaks the
 * `never` check at compile time. No branch offers a download: the badge is text and an icon only.
 */
function describeStatus(status: BackupJobStatusOrUnknown): StatusView {
  switch (status) {
    case 'queued':
      return { label: 'Queued', icon: Clock, tone: 'badge-ghost' };
    case 'running':
      return { label: 'Running', icon: Loader, tone: 'badge-info' };
    case 'succeeded':
      return { label: 'Succeeded', icon: CheckCircle2, tone: 'badge-success' };
    case 'failed':
      return { label: 'Failed', icon: XCircle, tone: 'badge-error' };
    case 'recovery-required':
      return { label: 'Recovery required', icon: AlertTriangle, tone: 'badge-warning' };
    case 'incomplete':
      return {
        label: 'Incomplete',
        icon: MinusCircle,
        tone: 'badge-warning',
        note: 'synthetic, database-only, not restorable',
      };
    case 'unknown':
      return { label: 'Unknown status', icon: CircleHelp, tone: 'badge-neutral' };
    default: {
      const unreachable: never = status;
      return unreachable;
    }
  }
}

/** F-45: the job status as visible text plus a decorative icon, never colour or icon alone (A11Y). */
export default function JobStatusBadge({ status, detailed = false }: JobStatusBadgeProps) {
  const { label, icon: Icon, tone, note } = describeStatus(status);
  return (
    <span className="inline-flex flex-wrap items-center gap-2">
      <span className={`badge gap-1 ${tone}`}>
        <Icon className="w-3 h-3" aria-hidden="true" />
        <span>{label}</span>
      </span>
      {detailed && note && <span className="text-xs text-base-content/70">({note})</span>}
    </span>
  );
}
