import { useRef, useState } from 'react';
import {
  BACKUP_FIXTURE_IDS,
  previewBackupPreflight,
  previewBackupRestoreConfirmation,
  type BackupFixtureId,
  type BackupPreflightPreview,
  type BackupRestoreConfirmation,
  type BackupRestoreDecision,
} from '@/domains/backup';

export interface SyntheticPreviewPanelProps {
  token: string | null;
}

const FIXTURE_OPTION_LABEL: Record<BackupFixtureId, string> = {
  'fixture-valid': 'Valid package',
  'fixture-incompatible': 'Incompatible package',
  'fixture-corrupt': 'Corrupt package',
};

const RESULT_LABEL: Record<BackupPreflightPreview['result'], string> = {
  compatible: 'Compatible',
  incompatible: 'Incompatible',
  corrupt: 'Corrupt',
};

type Identification = Pick<
  BackupPreflightPreview,
  'fixture_label' | 'target_instance_label' | 'target_environment' | 'data_scope'
>;

function isFixtureId(value: string): value is BackupFixtureId {
  return (BACKUP_FIXTURE_IDS as readonly string[]).includes(value);
}

/**
 * F-47: a labelled synthetic preview of the preflight and restore confirmation (IF-09, IF-10). It names the
 * fixture package, the target instance and the data in scope from the response identification fields
 * (AC-004 S1), offers confirm or cancel (AC-004 S2) and shows upload as unavailable. Nothing is uploaded,
 * downloaded, recorded or restored: the fixture id is chosen from a closed list and the server targets only
 * the configured instance. Server strings render as React text only (T-10). The failure copy is fixed.
 */
export default function SyntheticPreviewPanel({ token }: SyntheticPreviewPanelProps) {
  const inFlight = useRef(false);
  const [busy, setBusy] = useState(false);
  const [failed, setFailed] = useState(false);
  const [fixtureId, setFixtureId] = useState<BackupFixtureId>('fixture-valid');
  const [preflight, setPreflight] = useState<BackupPreflightPreview | null>(null);
  const [decision, setDecision] = useState<BackupRestoreConfirmation | null>(null);
  const [announcement, setAnnouncement] = useState('');

  const run = async (action: () => Promise<void>) => {
    if (inFlight.current) return;
    inFlight.current = true;
    setBusy(true);
    setFailed(false);
    try {
      await action();
    } catch {
      setFailed(true);
      setAnnouncement('');
    } finally {
      inFlight.current = false;
      setBusy(false);
    }
  };

  const runPreflight = () =>
    run(async () => {
      const result = await previewBackupPreflight(token, fixtureId);
      setPreflight(result);
      setDecision(null);
      setAnnouncement('Preflight finished.');
    });

  const decide = (choice: BackupRestoreDecision) =>
    run(async () => {
      const result = await previewBackupRestoreConfirmation(token, fixtureId, choice);
      setDecision(result);
      setAnnouncement('Decision received.');
    });

  const selectFixture = (value: string) => {
    if (!isFixtureId(value)) return;
    setFixtureId(value);
    setPreflight(null);
    setDecision(null);
    setAnnouncement('');
    setFailed(false);
  };

  const shown: Identification | null = decision ?? preflight;

  return (
    <section aria-labelledby="synthetic-preview-heading" className="space-y-3">
      <h2 id="synthetic-preview-heading" className="text-lg font-semibold">
        Synthetic restore preview
      </h2>
      <p className="text-sm text-base-content/70">
        This is a synthetic preview. No real package exists, so nothing is uploaded, downloaded or restored.
      </p>
      <button type="button" className="btn btn-sm" disabled>
        Upload package (unavailable)
      </button>
      <div className="flex flex-wrap items-end gap-3">
        <div className="form-control">
          <label htmlFor="synthetic-fixture-select" className="label-text text-sm">
            Synthetic fixture package
          </label>
          <select
            id="synthetic-fixture-select"
            className="select select-bordered select-sm"
            value={fixtureId}
            disabled={busy}
            onChange={(event) => selectFixture(event.target.value)}
          >
            {BACKUP_FIXTURE_IDS.map((id) => (
              <option key={id} value={id}>
                {FIXTURE_OPTION_LABEL[id]}
              </option>
            ))}
          </select>
        </div>
        <button type="button" className="btn btn-primary btn-sm" disabled={busy} onClick={() => void runPreflight()}>
          {busy && <span className="loading loading-spinner loading-xs" aria-hidden="true" />}
          Run preflight
        </button>
      </div>
      <div role="status" aria-live="polite" className="sr-only">
        {announcement}
      </div>
      {failed && (
        <div role="alert" className="alert alert-error">
          <span>Unable to run the synthetic preview. Try again.</span>
        </div>
      )}
      {preflight && shown && (
        <section aria-labelledby="synthetic-confirmation-heading" className="space-y-3 rounded border border-base-300 p-3">
          <h3 id="synthetic-confirmation-heading" className="font-medium">
            Restore confirmation
          </h3>
          <dl className="grid grid-cols-1 sm:grid-cols-[max-content_1fr] gap-x-6 gap-y-1 text-sm">
            <dt className="font-medium">Fixture package</dt>
            <dd>{shown.fixture_label}</dd>
            <dt className="font-medium">Target instance</dt>
            <dd>{shown.target_instance_label}</dd>
            <dt className="font-medium">Target environment</dt>
            <dd>{shown.target_environment}</dd>
            <dt className="font-medium">Data in scope</dt>
            <dd>
              <ul className="list-disc pl-5">
                {shown.data_scope.map((item, index) => (
                  <li key={`${index}-${item}`}>{item}</li>
                ))}
              </ul>
            </dd>
            <dt className="font-medium">Preflight result</dt>
            <dd>{RESULT_LABEL[preflight.result]}</dd>
          </dl>
          <p className="text-sm">{decision ? decision.message : preflight.impact_summary}</p>
          {decision ? (
            <p className="text-sm font-medium">
              {decision.decision === 'cancel' ? 'Cancelled. Nothing was changed.' : 'Confirmed as a preview.'} Restore not
              executed.
            </p>
          ) : preflight.result === 'compatible' ? (
            <div className="flex gap-2">
              <button type="button" className="btn btn-warning btn-sm" disabled={busy} onClick={() => void decide('confirm')}>
                Confirm restore preview
              </button>
              <button type="button" className="btn btn-sm" disabled={busy} onClick={() => void decide('cancel')}>
                Cancel
              </button>
            </div>
          ) : (
            <p className="text-sm font-medium">This package cannot be restored.</p>
          )}
        </section>
      )}
    </section>
  );
}
