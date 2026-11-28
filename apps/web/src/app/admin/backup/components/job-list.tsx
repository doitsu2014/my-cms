import type { BackupJob } from '@/domains/backup';
import JobStatusBadge from './job-status-badge';

export interface JobListProps {
  jobs: BackupJob[];
  phase: 'loading' | 'ready' | 'error';
  /** True while the hook is scheduling further polls. */
  polling: boolean;
  /** True when polling stopped because a status stayed unknown for the allowed number of polls. */
  exhausted: boolean;
  selectedId: string | null;
  onSelect: (id: string) => void;
  onRefresh?: () => void;
}

/** One polite announcement for the whole list, so assistive technology hears changes in plain text (A11Y). */
function announcement({ jobs, phase, polling, exhausted }: Pick<JobListProps, 'jobs' | 'phase' | 'polling' | 'exhausted'>): string {
  if (phase === 'loading' && jobs.length === 0) return 'Loading backup jobs.';
  if (jobs.length === 0) return phase === 'error' ? '' : 'No backup jobs yet.';
  if (exhausted) return 'Automatic updates have stopped.';
  return polling ? 'Updating automatically every 2 seconds.' : 'Up to date. Automatic updates have stopped.';
}

/**
 * F-46: the job list. Selecting a job is a real button (keyboard operable) and the current job carries
 * `aria-current`. Every server string is a React text child (T-10, TS-18) and no row offers a download.
 */
export default function JobList({ jobs, phase, polling, exhausted, selectedId, onSelect, onRefresh }: JobListProps) {
  return (
    <div className="space-y-3">
      <div role="status" aria-live="polite" className="text-sm text-base-content/70">
        {announcement({ jobs, phase, polling, exhausted })}
      </div>
      {phase === 'error' && !exhausted && (
        <div role="alert" className="alert alert-warning">
          <span>Unable to refresh backup jobs. Retrying automatically.</span>
        </div>
      )}
      {exhausted && (
        <div role="alert" className="alert alert-warning">
          <span>A job status could not be determined. Automatic updates have stopped.</span>
          {onRefresh && (
            <button type="button" className="btn btn-sm" onClick={onRefresh}>
              Refresh
            </button>
          )}
        </div>
      )}
      {jobs.length > 0 && (
        <div className="overflow-x-auto">
          <table className="table table-sm" aria-label="Backup jobs">
            <thead>
              <tr>
                <th scope="col">Reference</th>
                <th scope="col">Status</th>
                <th scope="col">Instance</th>
                <th scope="col">Created</th>
              </tr>
            </thead>
            <tbody>
              {jobs.map((job) => (
                <tr key={job.id}>
                  <td>
                    <button
                      type="button"
                      className="link link-hover break-all text-left"
                      aria-current={job.id === selectedId ? 'true' : undefined}
                      onClick={() => onSelect(job.id)}
                    >
                      {job.id}
                    </button>
                  </td>
                  <td>
                    <JobStatusBadge status={job.status} />
                  </td>
                  <td>{job.instance_label}</td>
                  <td>{job.created_at}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
