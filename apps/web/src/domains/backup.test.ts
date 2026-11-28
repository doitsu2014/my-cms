import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import {
  BackupApiError,
  backupInstancesResponseSchema,
  backupJobListResponseSchema,
  backupJobSchema,
  backupJobStatusSchema,
  backupStatusSchema,
  createBackupJob,
  getBackupJob,
  getBackupStatus,
  isTerminalBackupJobStatus,
  listBackupInstances,
  listBackupJobs,
} from './backup';

vi.mock('@/config/runtime-config', () => ({
  config: () => ({ restApiUrl: 'https://api.example.test/', graphqlApiUrl: '', supabaseUrl: '', supabaseAnonKey: '' }),
}));

const JOB_ID = '0b6f0f0e-6a9c-4f51-8f3c-3d2a1c9e7a10';

const queuedJob = {
  id: JOB_ID,
  instance_id: 'local-1',
  instance_label: 'Local development instance',
  operation: 'backup_db_slice',
  status: 'queued',
  reason_code: null,
  synthetic: true,
  download_eligible: false,
  components: [],
  progress: null,
  correlation_id: 'c-7f3a9e12',
  created_at: '2026-11-22T10:00:00Z',
  started_at: null,
  finished_at: null,
};

const incompleteJob = {
  ...queuedJob,
  status: 'incomplete',
  reason_code: 'REQUIRED_COMPONENTS_UNAVAILABLE',
  components: [
    { name: 'database', state: 'synthetic_captured', reason_code: 'DB_SYNTHETIC_ONLY' },
    { name: 'auth', state: 'unavailable', reason_code: 'AUTH_NOT_CAPTURED' },
    { name: 'storage', state: 'unavailable', reason_code: 'STORAGE_NOT_CAPTURED' },
  ],
  progress: { completed: 1, total: 1 },
  started_at: '2026-11-22T10:00:01Z',
  finished_at: '2026-11-22T10:00:02Z',
};

const readyStatus = {
  state: 'ready',
  disabled_reason: null,
  db_healthy: true,
  admissions_open: true,
  synthetic_only: true,
  reconciliation: { last_completed_at: '2026-11-22T10:15:00Z', last_result: 'ok' },
  cleanup: { pending: 0, exhausted: 0, orphan_discovered: 0, ownership_incidents: 0 },
  escalation: { owner: 'project_owner', visible: false },
};

const disabledStatus = (reason: string | null) => ({
  ...readyStatus,
  state: 'disabled',
  disabled_reason: reason,
  admissions_open: false,
  reconciliation: null,
  cleanup: null,
});

const json = (body: unknown, init: ResponseInit = { status: 200 }) =>
  new Response(JSON.stringify(body), { headers: { 'Content-Type': 'application/json' }, ...init });

const fetchMock = vi.fn();

beforeEach(() => {
  fetchMock.mockReset();
  vi.stubGlobal('fetch', fetchMock);
});
afterEach(() => {
  vi.unstubAllGlobals();
});

function lastCall() {
  expect(fetchMock).toHaveBeenCalledTimes(1);
  const [url, options] = fetchMock.mock.calls[0] as [string, RequestInit & { headers: Record<string, string> }];
  return { url, options };
}

describe('TS-52 client: status schema', () => {
  it.each(['queued', 'running', 'succeeded', 'failed', 'recovery-required', 'incomplete'])(
    'accepts documented status %s unchanged',
    (status) => {
      expect(backupJobStatusSchema.parse(status)).toBe(status);
    },
  );

  it('maps an unknown status string to unknown instead of failing', () => {
    expect(backupJobStatusSchema.parse('archived')).toBe('unknown');
    expect(backupJobStatusSchema.parse('')).toBe('unknown');
  });

  it('rejects a status that is not a string', () => {
    for (const bad of [undefined, null, 3, {}, ['queued']]) {
      expect(backupJobStatusSchema.safeParse(bad).success).toBe(false);
    }
  });

  it('classifies terminal statuses exhaustively and treats unknown as non-terminal', () => {
    expect(isTerminalBackupJobStatus('queued')).toBe(false);
    expect(isTerminalBackupJobStatus('running')).toBe(false);
    expect(isTerminalBackupJobStatus('unknown')).toBe(false);
    for (const status of ['incomplete', 'failed', 'succeeded', 'recovery-required'] as const) {
      expect(isTerminalBackupJobStatus(status)).toBe(true);
    }
  });
});

describe('TS-52 client: job schema', () => {
  it('parses a queued job with empty components and null progress', () => {
    expect(backupJobSchema.parse(queuedJob)).toEqual(queuedJob);
  });

  it('parses an incomplete job with all three components and measured progress', () => {
    const parsed = backupJobSchema.parse(incompleteJob);
    expect(parsed.status).toBe('incomplete');
    expect(parsed.components).toHaveLength(3);
    expect(parsed.progress).toEqual({ completed: 1, total: 1 });
  });

  it('tolerates an unknown status inside a job and still marks it safe', () => {
    const parsed = backupJobSchema.parse({ ...queuedJob, status: 'archived' });
    expect(parsed.status).toBe('unknown');
    expect(parsed.download_eligible).toBe(false);
  });

  it('fails closed when a job claims to be downloadable or not synthetic', () => {
    expect(backupJobSchema.safeParse({ ...queuedJob, download_eligible: true }).success).toBe(false);
    expect(backupJobSchema.safeParse({ ...queuedJob, synthetic: false }).success).toBe(false);
  });

  it('rejects missing or mistyped required fields', () => {
    const withoutId: Record<string, unknown> = { ...queuedJob };
    delete withoutId.id;
    expect(backupJobSchema.safeParse(withoutId).success).toBe(false);
    expect(backupJobSchema.safeParse({ ...queuedJob, created_at: 5 }).success).toBe(false);
    expect(backupJobSchema.safeParse({ ...queuedJob, components: [{ name: 'network', state: 'unavailable', reason_code: null }] }).success).toBe(false);
    expect(backupJobSchema.safeParse({ ...queuedJob, components: [{ name: 'auth', state: 'done', reason_code: null }] }).success).toBe(false);
    expect(backupJobSchema.safeParse({ ...queuedJob, progress: { completed: -1, total: 1 } }).success).toBe(false);
  });

  it('parses list and instance envelopes strictly', () => {
    expect(backupJobListResponseSchema.parse({ jobs: [queuedJob, incompleteJob], next_cursor: null }).jobs).toHaveLength(2);
    expect(backupJobListResponseSchema.safeParse({ jobs: [queuedJob] }).success).toBe(false);
    const instances = {
      synthetic: true,
      instances: [{
        id: 'local-1', label: 'Local development instance', environment: 'local', operations: ['backup_db_slice'],
        capabilities: { create: true, upload: false, preflight: false, restore: false, download: false },
      }],
    };
    expect(backupInstancesResponseSchema.parse(instances)).toEqual(instances);
    expect(backupInstancesResponseSchema.parse({ synthetic: true, instances: [] }).instances).toEqual([]);
    expect(backupInstancesResponseSchema.safeParse({ ...instances, synthetic: false }).success).toBe(false);
  });
});

describe('TS-52 client: status object', () => {
  it('parses a ready status with null disabled_reason', () => {
    expect(backupStatusSchema.parse(readyStatus)).toEqual(readyStatus);
  });

  it.each(['NOT_CONFIGURED', 'NON_LOOPBACK_BIND', 'INVALID_CONFIG'])('accepts disabled_reason %s', (reason) => {
    const parsed = backupStatusSchema.parse(disabledStatus(reason));
    expect(parsed.state).toBe('disabled');
    expect(parsed.disabled_reason).toBe(reason);
    expect(parsed.cleanup).toBeNull();
    expect(parsed.reconciliation).toBeNull();
  });

  it('rejects a disabled_reason outside the closed set and a missing one', () => {
    expect(backupStatusSchema.safeParse(disabledStatus('SOMETHING_NEW')).success).toBe(false);
    const withoutReason: Record<string, unknown> = { ...disabledStatus(null) };
    delete withoutReason.disabled_reason;
    expect(backupStatusSchema.safeParse(withoutReason).success).toBe(false);
  });

  it('accepts every documented state and rejects others', () => {
    for (const state of ['ready', 'not_ready', 'degraded', 'disabled']) {
      expect(backupStatusSchema.safeParse({ ...readyStatus, state }).success).toBe(true);
    }
    expect(backupStatusSchema.safeParse({ ...readyStatus, state: 'on_fire' }).success).toBe(false);
  });

  it('requires synthetic_only to be true and counters to be non-negative integers', () => {
    expect(backupStatusSchema.safeParse({ ...readyStatus, synthetic_only: false }).success).toBe(false);
    expect(backupStatusSchema.safeParse({ ...readyStatus, cleanup: { ...readyStatus.cleanup, exhausted: -1 } }).success).toBe(false);
  });
});

describe('TS-52 client: REST calls', () => {
  it('lists instances with an authorised GET against the backup path', async () => {
    fetchMock.mockResolvedValue(json({ synthetic: true, instances: [] }));
    const result = await listBackupInstances('tok');
    const { url, options } = lastCall();
    expect(url).toBe('https://api.example.test/backup/instances');
    expect(options.headers.Authorization).toBe('Bearer tok');
    expect(options.method ?? 'GET').toBe('GET');
    expect(result.instances).toEqual([]);
  });

  it('creates a job with Idempotency-Key, JSON content type and only instance_id in the body', async () => {
    fetchMock.mockResolvedValue(json({ job: queuedJob }, { status: 202 }));
    const key = '6b0c1f52-3d34-4e0b-9a8c-0f1d2c3b4a59';
    const result = await createBackupJob('tok', 'local-1', key);
    const { url, options } = lastCall();
    expect(url).toBe('https://api.example.test/backup/jobs');
    expect(options.method).toBe('POST');
    expect(options.headers['Idempotency-Key']).toBe(key);
    expect(options.headers['Content-Type']).toBe('application/json');
    expect(options.headers.Authorization).toBe('Bearer tok');
    expect(JSON.parse(options.body as string)).toEqual({ instance_id: 'local-1' });
    expect(result.job.id).toBe(JOB_ID);
    expect(result.replayed).toBe(false);
  });

  it('reports an idempotent replay from the Idempotent-Replay header on a 200', async () => {
    fetchMock.mockResolvedValue(json({ job: queuedJob }, { status: 200, headers: { 'Idempotent-Replay': 'true', 'Content-Type': 'application/json' } }));
    const result = await createBackupJob('tok', 'local-1', 'k');
    expect(result.replayed).toBe(true);
    expect(result.job.status).toBe('queued');
  });

  it('lists jobs with limit and cursor query parameters only when supplied', async () => {
    fetchMock.mockResolvedValue(json({ jobs: [incompleteJob], next_cursor: 'abc=' }));
    const page = await listBackupJobs('tok', { limit: 20, cursor: 'a b+c=' });
    const { url } = lastCall();
    const parsed = new URL(url);
    expect(parsed.origin + parsed.pathname).toBe('https://api.example.test/backup/jobs');
    expect(parsed.searchParams.get('limit')).toBe('20');
    expect(parsed.searchParams.get('cursor')).toBe('a b+c=');
    expect(page.next_cursor).toBe('abc=');

    fetchMock.mockClear();
    fetchMock.mockResolvedValue(json({ jobs: [], next_cursor: null }));
    await listBackupJobs('tok');
    expect(lastCall().url).toBe('https://api.example.test/backup/jobs');
  });

  it('gets one job by encoded id and tolerates an unknown status', async () => {
    fetchMock.mockResolvedValue(json({ job: { ...queuedJob, status: 'archived' } }));
    const job = await getBackupJob('tok', 'a/b c');
    expect(lastCall().url).toBe('https://api.example.test/backup/jobs/a%2Fb%20c');
    expect(job.status).toBe('unknown');
  });

  it('gets the status object including a disabled state', async () => {
    fetchMock.mockResolvedValue(json(disabledStatus('NON_LOOPBACK_BIND')));
    const status = await getBackupStatus('tok');
    expect(lastCall().url).toBe('https://api.example.test/backup/status');
    expect(status.state).toBe('disabled');
    expect(status.disabled_reason).toBe('NON_LOOPBACK_BIND');
  });
});

describe('TS-52 client: error handling', () => {
  it('parses the crate error body into a typed BackupApiError', async () => {
    fetchMock.mockResolvedValue(json({
      error_code: 'Conflict',
      errors: ['Idempotency key was used for a different request.'],
      reason_code: 'IDEMPOTENCY_KEY_REUSED',
      correlation_id: 'c-2',
    }, { status: 409 }));
    const error = await createBackupJob('tok', 'local-1', 'k').catch((e: unknown) => e);
    expect(error).toBeInstanceOf(BackupApiError);
    expect(error).toEqual(expect.objectContaining({ status: 409, errorCode: 'Conflict', reasonCode: 'IDEMPOTENCY_KEY_REUSED', correlationId: 'c-2' }));
  });

  it('keeps BACKUP_DISABLED and BACKUP_NOT_READY 503 reasons', async () => {
    fetchMock.mockResolvedValue(json({ error_code: 'Unavailable', errors: ['Backup is disabled.'], reason_code: 'BACKUP_DISABLED', correlation_id: 'c-9' }, { status: 503 }));
    await expect(listBackupInstances('tok')).rejects.toEqual(expect.objectContaining({ status: 503, reasonCode: 'BACKUP_DISABLED' }));
  });

  it('falls back to the HTTP status for non-JSON error bodies such as a plain-text 413', async () => {
    fetchMock.mockResolvedValue(new Response('payload too large', { status: 413 }));
    const error = await createBackupJob('tok', 'local-1', 'k').catch((e: unknown) => e);
    expect(error).toBeInstanceOf(BackupApiError);
    expect(error).toEqual(expect.objectContaining({ status: 413 }));
    expect((error as BackupApiError).reasonCode).toBeUndefined();
  });

  it('rejects a success response whose body does not match the contract', async () => {
    fetchMock.mockResolvedValue(json({ job: { ...queuedJob, download_eligible: true } }, { status: 202 }));
    const error = await createBackupJob('tok', 'local-1', 'k').catch((e: unknown) => e);
    expect(error).toBeInstanceOf(BackupApiError);
    expect((error as BackupApiError).reasonCode).toBe('INVALID_RESPONSE');

    fetchMock.mockResolvedValue(new Response('<html>', { status: 200 }));
    await expect(getBackupStatus('tok')).rejects.toEqual(expect.objectContaining({ reasonCode: 'INVALID_RESPONSE' }));
  });

  it('does not copy server error strings into the error message', async () => {
    fetchMock.mockResolvedValue(json({ error_code: 'Forbidden', errors: ['<img src=x onerror=alert(1)>'], reason_code: 'ROLE_REQUIRED', correlation_id: 'c-1' }, { status: 403 }));
    const error = (await getBackupStatus('tok').catch((e: unknown) => e)) as BackupApiError;
    expect(error.message).not.toContain('onerror');
    expect(error.reasonCode).toBe('ROLE_REQUIRED');
  });
});
