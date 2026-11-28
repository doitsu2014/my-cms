import { Home } from 'lucide-react';
import { useCallback, useEffect, useState } from 'react';
import Breadcrumbs from '@/app/admin/components/my-breadcrumbs';
import { useAuth } from '@/auth/AuthContext';
import { listBackupInstances, type BackupInstance, type BackupJob } from '@/domains/backup';
import BackupStatusBanner from './components/backup-status-banner';
import InstanceSelector from './components/instance-selector';
import JobCreatePanel from './components/job-create-panel';
import JobDetail from './components/job-detail';
import JobList from './components/job-list';
import SyntheticPreviewPanel from './components/synthetic-preview-panel';
import { useBackupJobs } from './hooks/use-backup-jobs';
import { useBackupStatus } from './hooks/use-backup-status';

export default function AdminBackupPage() {
  const { token } = useAuth();
  const [instances, setInstances] = useState<BackupInstance[]>([]);
  const [state, setState] = useState<'loading' | 'ready' | 'error'>('loading');
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [created, setCreated] = useState<BackupJob | null>(null);
  const [selectedJobId, setSelectedJobId] = useState<string | null>(null);
  const backupJobs = useBackupJobs(token);
  const backupStatus = useBackupStatus(token);
  const { refresh: refreshJobs } = backupJobs;
  const { refresh: refreshStatus } = backupStatus;

  const load = useCallback(async () => {
    setState('loading');
    try {
      const { instances: listed } = await listBackupInstances(token);
      setInstances(listed);
      // The API lists zero or one instance (IF-01); a single instance is selected for the administrator.
      setSelectedId(listed.length === 1 ? listed[0].id : null);
      setState('ready');
    } catch {
      setInstances([]);
      setSelectedId(null);
      setState('error');
    }
  }, [token]);

  useEffect(() => {
    void load();
  }, [load]);

  // Submission needs a listed, selected instance; an empty, loading or failed list leaves this null.
  const submitInstanceId = state === 'ready' && instances.some((item) => item.id === selectedId) ? selectedId : null;

  const handleCreated = useCallback(
    (job: BackupJob) => {
      setCreated(job);
      setSelectedJobId(job.id);
      // A new job is non-terminal: reload the list (which restarts polling) and the status counters.
      refreshJobs();
      refreshStatus();
    },
    [refreshJobs, refreshStatus],
  );

  const selectedJob = backupJobs.jobs.find((job) => job.id === selectedJobId) ?? null;

  return (
    <div className="space-y-6">
      <Breadcrumbs items={[{ label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> }, { label: 'Backup' }]} />
      <div>
        <h1 className="text-2xl font-bold">Backup</h1>
        <p className="text-base-content/60 text-sm mt-1">Create a synthetic backup job for an eligible instance.</p>
      </div>
      <BackupStatusBanner status={backupStatus.status} phase={backupStatus.phase} />
      <div className="card bg-base-200">
        <div className="card-body space-y-4">
          <InstanceSelector
            instances={instances}
            state={state}
            selectedId={selectedId}
            onSelect={setSelectedId}
            onRetry={() => void load()}
          />
          <JobCreatePanel token={token} instanceId={submitInstanceId} onCreated={handleCreated} />
          {created && (
            <div role="status" className="alert alert-success">
              <span>Backup job created. Reference: {created.id}</span>
            </div>
          )}
        </div>
      </div>
      <div className="card bg-base-200">
        <div className="card-body space-y-4">
          <h2 className="text-lg font-semibold">Backup jobs</h2>
          <JobList
            jobs={backupJobs.jobs}
            phase={backupJobs.phase}
            polling={backupJobs.polling}
            exhausted={backupJobs.exhausted}
            selectedId={selectedJobId}
            onSelect={setSelectedJobId}
            onRefresh={refreshJobs}
          />
          {selectedJob && <JobDetail job={selectedJob} />}
        </div>
      </div>
      <div className="card bg-base-200">
        <div className="card-body">
          <SyntheticPreviewPanel token={token} />
        </div>
      </div>
    </div>
  );
}
