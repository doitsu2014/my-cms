import { z } from 'zod';
import { authenticatedFetch, getBackupApiUrl } from '@/config/api.config';

/**
 * REST client for the backup surface (F-41, D-13). The wire contract is
 * interfaces-and-contracts IF-01..IF-04 and IF-11: snake_case fields, bare success bodies and the
 * crate-local `{error_code, errors, reason_code, correlation_id}` error body (D-11). It deliberately does
 * not reuse the `{message, data}` / `{errorCode, errors}` parsers of the other domains.
 */

/** Every documented status value (D-10): the existing five plus `incomplete`. */
export const KNOWN_BACKUP_JOB_STATUSES = [
  'queued',
  'running',
  'succeeded',
  'failed',
  'recovery-required',
  'incomplete',
] as const;
export type BackupJobStatus = (typeof KNOWN_BACKUP_JOB_STATUSES)[number];
/** What the client hands to the UI: a documented value, or `unknown` for any value it does not recognise. */
export type BackupJobStatusOrUnknown = BackupJobStatus | 'unknown';

/** Closed `disabled_reason` vocabulary (IF-11). `null` is expressed by the nullable schema, not a member. */
export const BACKUP_DISABLED_REASONS = ['NOT_CONFIGURED', 'NON_LOOPBACK_BIND', 'INVALID_CONFIG'] as const;
export type BackupDisabledReason = (typeof BACKUP_DISABLED_REASONS)[number];

export const BACKUP_STATUS_STATES = ['ready', 'not_ready', 'degraded', 'disabled'] as const;

/**
 * A documented status passes through; any other string maps to `unknown` (forward-compatible, never
 * downloadable and never terminal success). A non-string still fails: that is a contract violation.
 */
export const backupJobStatusSchema = z.union([
  z.enum(KNOWN_BACKUP_JOB_STATUSES),
  z.string().transform((): 'unknown' => 'unknown'),
]);

const backupComponentSchema = z.object({
  name: z.enum(['database', 'auth', 'storage']),
  state: z.enum(['synthetic_captured', 'unavailable']),
  reason_code: z.string().nullable(),
});

const counter = z.number().int().nonnegative();

export const backupJobSchema = z.object({
  id: z.string().min(1),
  instance_id: z.string(),
  instance_label: z.string(),
  operation: z.string(),
  status: backupJobStatusSchema,
  reason_code: z.string().nullable(),
  // Fail closed: a job that claims to be real or downloadable violates the U1 contract.
  synthetic: z.literal(true),
  download_eligible: z.literal(false),
  components: z.array(backupComponentSchema),
  progress: z.object({ completed: counter, total: counter }).nullable(),
  correlation_id: z.string(),
  created_at: z.string().min(1),
  started_at: z.string().nullable(),
  finished_at: z.string().nullable(),
});

export const backupInstanceSchema = z.object({
  id: z.string().min(1),
  label: z.string(),
  environment: z.string(),
  operations: z.array(z.string()),
  capabilities: z.object({
    create: z.boolean(),
    upload: z.boolean(),
    preflight: z.boolean(),
    restore: z.boolean(),
    download: z.boolean(),
  }),
});

export const backupInstancesResponseSchema = z.object({
  synthetic: z.literal(true),
  instances: z.array(backupInstanceSchema),
});

export const backupJobListResponseSchema = z.object({
  jobs: z.array(backupJobSchema),
  next_cursor: z.string().nullable(),
});

export const backupStatusSchema = z.object({
  state: z.enum(BACKUP_STATUS_STATES),
  disabled_reason: z.enum(BACKUP_DISABLED_REASONS).nullable(),
  db_healthy: z.boolean(),
  admissions_open: z.boolean(),
  synthetic_only: z.literal(true),
  reconciliation: z
    .object({ last_completed_at: z.string().nullable(), last_result: z.string().nullable() })
    .nullable(),
  cleanup: z
    .object({ pending: counter, exhausted: counter, orphan_discovered: counter, ownership_incidents: counter })
    .nullable(),
  escalation: z.object({ owner: z.string(), visible: z.boolean() }),
});

export type BackupComponent = z.infer<typeof backupComponentSchema>;
export type BackupJob = z.infer<typeof backupJobSchema>;
export type BackupInstance = z.infer<typeof backupInstanceSchema>;
export type BackupInstancesResponse = z.infer<typeof backupInstancesResponseSchema>;
export type BackupJobListResponse = z.infer<typeof backupJobListResponseSchema>;
export type BackupStatus = z.infer<typeof backupStatusSchema>;

const errorBodySchema = z.object({
  error_code: z.string().optional(),
  reason_code: z.string().optional(),
  correlation_id: z.string().optional(),
});

/** Reason code used when a 2xx body does not match the contract. */
export const INVALID_RESPONSE = 'INVALID_RESPONSE';

/**
 * Failure of a backup request. The message is fixed text: server `errors` strings are never copied in, so
 * the UI branches on `reasonCode` and renders its own copy.
 */
export class BackupApiError extends Error {
  constructor(
    public readonly status: number,
    public readonly errorCode?: string,
    public readonly reasonCode?: string,
    public readonly correlationId?: string,
  ) {
    super(`Backup request failed (HTTP ${status})`);
    this.name = 'BackupApiError';
  }
}

async function readJson(response: Response): Promise<unknown> {
  try {
    return await response.json();
  } catch {
    return undefined;
  }
}

async function failure(response: Response): Promise<BackupApiError> {
  const parsed = errorBodySchema.safeParse(await readJson(response));
  const body = parsed.success ? parsed.data : {};
  return new BackupApiError(response.status, body.error_code, body.reason_code, body.correlation_id);
}

async function parseResponse<S extends z.ZodType>(response: Response, schema: S): Promise<z.output<S>> {
  if (!response.ok) throw await failure(response);
  const parsed = schema.safeParse(await readJson(response));
  if (!parsed.success) throw new BackupApiError(response.status, undefined, INVALID_RESPONSE);
  return parsed.data;
}

const jobEnvelopeSchema = z.object({ job: backupJobSchema });

/**
 * `incomplete`, `failed` and the retained `succeeded` / `recovery-required` need no further polling. `unknown`
 * is non-terminal (IFC 3.2: polled a bounded number of times by the caller).
 */
export function isTerminalBackupJobStatus(status: BackupJobStatusOrUnknown): boolean {
  switch (status) {
    case 'queued':
    case 'running':
    case 'unknown':
      return false;
    case 'incomplete':
    case 'failed':
    case 'succeeded':
    case 'recovery-required':
      return true;
    default: {
      const unreachable: never = status;
      return unreachable;
    }
  }
}

export async function listBackupInstances(token: string | null): Promise<BackupInstancesResponse> {
  const response = await authenticatedFetch(getBackupApiUrl('/instances'), token, { cache: 'no-store' });
  return parseResponse(response, backupInstancesResponseSchema);
}

export async function createBackupJob(
  token: string | null,
  instanceId: string,
  idempotencyKey: string,
): Promise<{ job: BackupJob; replayed: boolean }> {
  const response = await authenticatedFetch(getBackupApiUrl('/jobs'), token, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', 'Idempotency-Key': idempotencyKey },
    body: JSON.stringify({ instance_id: instanceId }),
  });
  const { job } = await parseResponse(response, jobEnvelopeSchema);
  return { job, replayed: response.headers.get('Idempotent-Replay') === 'true' };
}

export async function listBackupJobs(
  token: string | null,
  options: { limit?: number; cursor?: string } = {},
): Promise<BackupJobListResponse> {
  const query = new URLSearchParams();
  if (options.limit !== undefined) query.set('limit', String(options.limit));
  if (options.cursor !== undefined) query.set('cursor', options.cursor);
  const suffix = query.size > 0 ? `?${query.toString()}` : '';
  const response = await authenticatedFetch(getBackupApiUrl(`/jobs${suffix}`), token, { cache: 'no-store' });
  return parseResponse(response, backupJobListResponseSchema);
}

export async function getBackupJob(token: string | null, jobId: string): Promise<BackupJob> {
  const response = await authenticatedFetch(getBackupApiUrl(`/jobs/${encodeURIComponent(jobId)}`), token, {
    cache: 'no-store',
  });
  return (await parseResponse(response, jobEnvelopeSchema)).job;
}

export async function getBackupStatus(token: string | null): Promise<BackupStatus> {
  const response = await authenticatedFetch(getBackupApiUrl('/status'), token, { cache: 'no-store' });
  return parseResponse(response, backupStatusSchema);
}

/** Trusted synthetic fixture ids (IF-09). The API accepts nothing else; no upload or path exists in U1. */
export const BACKUP_FIXTURE_IDS = ['fixture-valid', 'fixture-incompatible', 'fixture-corrupt'] as const;
export type BackupFixtureId = (typeof BACKUP_FIXTURE_IDS)[number];
export type BackupRestoreDecision = 'confirm' | 'cancel';

/** IF-09 response: identification fields of a synthetic preflight (interfaces-and-contracts 2.9). */
export const backupPreflightPreviewSchema = z.object({
  synthetic: z.literal(true),
  result: z.enum(['compatible', 'incompatible', 'corrupt']),
  fixture_label: z.string(),
  target_instance_id: z.string(),
  target_instance_label: z.string(),
  target_environment: z.string(),
  data_scope: z.array(z.string()),
  impact_summary: z.string(),
});

/** IF-10 response. `executed` is always false: restore execution is not authorized in U1. */
export const backupRestoreConfirmationSchema = z.object({
  synthetic: z.literal(true),
  executed: z.literal(false),
  decision: z.enum(['confirm', 'cancel']),
  fixture_label: z.string(),
  target_instance_id: z.string(),
  target_instance_label: z.string(),
  target_environment: z.string(),
  data_scope: z.array(z.string()),
  message: z.string(),
});

export type BackupPreflightPreview = z.infer<typeof backupPreflightPreviewSchema>;
export type BackupRestoreConfirmation = z.infer<typeof backupRestoreConfirmationSchema>;

/** IF-09 `POST /backup/previews/preflight`. Sends only a trusted fixture id: no upload, path or target. */
export async function previewBackupPreflight(token: string | null, fixtureId: BackupFixtureId): Promise<BackupPreflightPreview> {
  const response = await authenticatedFetch(getBackupApiUrl('/previews/preflight'), token, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ fixture_id: fixtureId }),
  });
  return parseResponse(response, backupPreflightPreviewSchema);
}

/** IF-10 `POST /backup/previews/restore-confirmation`. Stateless on the server: nothing is executed. */
export async function previewBackupRestoreConfirmation(
  token: string | null,
  fixtureId: BackupFixtureId,
  decision: BackupRestoreDecision,
): Promise<BackupRestoreConfirmation> {
  const response = await authenticatedFetch(getBackupApiUrl('/previews/restore-confirmation'), token, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ fixture_id: fixtureId, decision }),
  });
  return parseResponse(response, backupRestoreConfirmationSchema);
}
