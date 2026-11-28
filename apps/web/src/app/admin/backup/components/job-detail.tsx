import type { BackupComponent, BackupJob } from '@/domains/backup';
import JobStatusBadge from './job-status-badge';

const COMPONENT_STATE_LABEL: Record<BackupComponent['state'], string> = {
  synthetic_captured: 'Synthetic data captured',
  unavailable: 'Unavailable',
};

const NONE = 'None';

/**
 * F-45: one backup job. Every server-supplied value (labels, codes, ids, timestamps) is rendered as a React
 * text child, never as markup (T-10, TS-18). There is no download control for any status: U1 jobs are
 * synthetic and database-only (T-06, TS-12a).
 */
export default function JobDetail({ job }: { job: BackupJob }) {
  return (
    <section aria-labelledby="backup-job-details-heading" className="space-y-3">
      <h2 id="backup-job-details-heading" className="text-lg font-semibold">
        Backup job details
      </h2>
      <dl className="grid grid-cols-1 sm:grid-cols-[max-content_1fr] gap-x-6 gap-y-1 text-sm">
        <dt className="font-medium">Reference</dt>
        <dd className="break-all">{job.id}</dd>
        <dt className="font-medium">Status</dt>
        <dd>
          <JobStatusBadge status={job.status} detailed />
        </dd>
        <dt className="font-medium">Reason</dt>
        <dd>{job.reason_code ?? NONE}</dd>
        <dt className="font-medium">Instance</dt>
        <dd>{job.instance_label}</dd>
        <dt className="font-medium">Operation</dt>
        <dd>{job.operation}</dd>
        <dt className="font-medium">Progress</dt>
        <dd>{job.progress ? `${job.progress.completed} of ${job.progress.total}` : 'Not measured'}</dd>
        <dt className="font-medium">Created</dt>
        <dd>{job.created_at}</dd>
        <dt className="font-medium">Started</dt>
        <dd>{job.started_at ?? 'Not started'}</dd>
        <dt className="font-medium">Finished</dt>
        <dd>{job.finished_at ?? 'Not finished'}</dd>
        <dt className="font-medium">Correlation</dt>
        <dd className="break-all">{job.correlation_id}</dd>
      </dl>
      <div>
        <h3 className="text-sm font-medium" id="backup-job-components-heading">
          Components
        </h3>
        {job.components.length === 0 ? (
          <p className="text-sm text-base-content/70">No component results recorded yet.</p>
        ) : (
          <ul aria-labelledby="backup-job-components-heading" className="list-disc pl-5 text-sm">
            {job.components.map((component) => (
              <li key={component.name}>
                <span className="font-medium">{component.name}</span>: {COMPONENT_STATE_LABEL[component.state]}
                {component.reason_code && <> (<span>{component.reason_code}</span>)</>}
              </li>
            ))}
          </ul>
        )}
      </div>
      <p className="text-sm text-base-content/70">
        Download is not available. This job is synthetic and contains no restorable data.
      </p>
    </section>
  );
}
