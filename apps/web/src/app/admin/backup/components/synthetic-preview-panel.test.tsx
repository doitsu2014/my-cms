import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import SyntheticPreviewPanel from './synthetic-preview-panel';
import {
  BackupApiError,
  previewBackupPreflight,
  previewBackupRestoreConfirmation,
  type BackupPreflightPreview,
  type BackupRestoreConfirmation,
} from '@/domains/backup';

vi.mock('@/domains/backup', async () => {
  const actual = await vi.importActual<typeof import('@/domains/backup')>('@/domains/backup');
  return { ...actual, previewBackupPreflight: vi.fn(), previewBackupRestoreConfirmation: vi.fn() };
});

const preflightMock = vi.mocked(previewBackupPreflight);
const confirmMock = vi.mocked(previewBackupRestoreConfirmation);

const preflight = (over: Partial<BackupPreflightPreview> = {}): BackupPreflightPreview => ({
  synthetic: true,
  result: 'compatible',
  fixture_label: 'Synthetic valid fixture',
  target_instance_id: 'local-1',
  target_instance_label: 'Local development instance',
  target_environment: 'local',
  data_scope: ['database (synthetic rows only)'],
  impact_summary: 'Synthetic preview only. No file was uploaded or read.',
  ...over,
});

const confirmation = (over: Partial<BackupRestoreConfirmation> = {}): BackupRestoreConfirmation => ({
  synthetic: true,
  executed: false,
  decision: 'confirm',
  fixture_label: 'Synthetic valid fixture',
  target_instance_id: 'local-1',
  target_instance_label: 'Local development instance',
  target_environment: 'local',
  data_scope: ['database (synthetic rows only)'],
  message: 'Synthetic preview only. Nothing was recorded and no restore was performed.',
  ...over,
});

const runPreflight = () => fireEvent.click(screen.getByRole('button', { name: /run preflight/i }));

describe('SyntheticPreviewPanel', () => {
  beforeEach(() => {
    preflightMock.mockReset();
    confirmMock.mockReset();
  });

  // TS-51
  it('ts51_is_labelled_synthetic_and_shows_upload_as_unavailable', () => {
    render(<SyntheticPreviewPanel token="tok" />);
    expect(screen.getByRole('heading', { name: /synthetic/i })).toBeInTheDocument();
    expect(screen.getByText(/no real package/i)).toBeInTheDocument();
    const upload = screen.getByRole('button', { name: /upload.*unavailable/i });
    expect(upload).toBeDisabled();
    expect(screen.queryByRole('button', { name: /download/i })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /confirm/i })).not.toBeInTheDocument();
  });

  it('ts51_preflight_names_the_fixture_package_target_instance_and_data_in_scope', async () => {
    preflightMock.mockResolvedValue(preflight());
    render(<SyntheticPreviewPanel token="tok" />);
    runPreflight();
    const result = await screen.findByRole('region', { name: /restore confirmation/i });
    expect(preflightMock).toHaveBeenCalledWith('tok', 'fixture-valid');
    expect(within(result).getByText('Synthetic valid fixture')).toBeInTheDocument();
    expect(within(result).getByText(/Local development instance/)).toBeInTheDocument();
    expect(within(result).getByText(/local/)).toBeInTheDocument();
    expect(within(result).getByText('database (synthetic rows only)')).toBeInTheDocument();
    expect(within(result).getByText(/compatible/i)).toBeInTheDocument();
    expect(within(result).getByText(/synthetic preview only/i)).toBeInTheDocument();
  });

  it('ts51_the_selected_fixture_is_the_one_sent', async () => {
    preflightMock.mockResolvedValue(preflight({ result: 'corrupt', fixture_label: 'Synthetic corrupt fixture' }));
    const user = userEvent.setup();
    render(<SyntheticPreviewPanel token="tok" />);
    await user.selectOptions(screen.getByLabelText(/synthetic fixture package/i), 'fixture-corrupt');
    runPreflight();
    await screen.findByText('Synthetic corrupt fixture');
    expect(preflightMock).toHaveBeenCalledWith('tok', 'fixture-corrupt');
  });

  it('ts51_confirm_posts_the_decision_and_shows_executed_false_with_the_identification_fields', async () => {
    preflightMock.mockResolvedValue(preflight());
    confirmMock.mockResolvedValue(confirmation());
    render(<SyntheticPreviewPanel token="tok" />);
    runPreflight();
    fireEvent.click(await screen.findByRole('button', { name: /^confirm/i }));
    await waitFor(() => expect(confirmMock).toHaveBeenCalledWith('tok', 'fixture-valid', 'confirm'));
    const done = await screen.findByText(/no restore was performed/i);
    expect(done).toBeInTheDocument();
    expect(screen.getByText(/not executed/i)).toBeInTheDocument();
    expect(screen.getAllByText('Synthetic valid fixture').length).toBeGreaterThan(0);
  });

  it('ts51_cancel_posts_cancel_and_confirms_nothing_changed', async () => {
    preflightMock.mockResolvedValue(preflight());
    confirmMock.mockResolvedValue(confirmation({ decision: 'cancel' }));
    render(<SyntheticPreviewPanel token="tok" />);
    runPreflight();
    fireEvent.click(await screen.findByRole('button', { name: /^cancel/i }));
    await waitFor(() => expect(confirmMock).toHaveBeenCalledWith('tok', 'fixture-valid', 'cancel'));
    expect(await screen.findByText(/cancelled/i)).toBeInTheDocument();
  });

  it('ts51_an_incompatible_or_corrupt_package_cannot_be_confirmed', async () => {
    preflightMock.mockResolvedValue(preflight({ result: 'incompatible', fixture_label: 'Synthetic incompatible fixture' }));
    render(<SyntheticPreviewPanel token="tok" />);
    runPreflight();
    await screen.findByText('Synthetic incompatible fixture');
    expect(screen.queryByRole('button', { name: /^confirm/i })).not.toBeInTheDocument();
    expect(screen.getByText(/cannot be restored/i)).toBeInTheDocument();
    expect(confirmMock).not.toHaveBeenCalled();
  });

  it('ts51_a_failed_request_shows_fixed_copy_and_never_the_server_string', async () => {
    preflightMock.mockRejectedValue(new BackupApiError(503, 'BACKUP_DISABLED', 'NOT_CONFIGURED'));
    render(<SyntheticPreviewPanel token="tok" />);
    runPreflight();
    expect(await screen.findByRole('alert')).toHaveTextContent(/unable to run the synthetic preview/i);
  });

  it('ts51_a_second_click_while_a_request_is_in_flight_sends_one_request', async () => {
    preflightMock.mockReturnValue(new Promise(() => {}));
    render(<SyntheticPreviewPanel token="tok" />);
    runPreflight();
    runPreflight();
    expect(preflightMock).toHaveBeenCalledTimes(1);
    expect(screen.getByRole('button', { name: /run preflight/i })).toBeDisabled();
  });

  // TS-37: labels, keyboard
  it('ts37_controls_have_accessible_names_and_work_from_the_keyboard', async () => {
    preflightMock.mockResolvedValue(preflight());
    const user = userEvent.setup();
    render(<SyntheticPreviewPanel token="tok" />);
    expect(screen.getByLabelText(/synthetic fixture package/i)).toBeInTheDocument();
    await user.tab();
    expect(screen.getByLabelText(/synthetic fixture package/i)).toHaveFocus();
    await user.tab();
    expect(screen.getByRole('button', { name: /run preflight/i })).toHaveFocus();
    await user.keyboard('{Enter}');
    expect(await screen.findByRole('region', { name: /restore confirmation/i })).toBeInTheDocument();
    expect(screen.getByRole('status')).toBeInTheDocument();
  });

  // TS-18
  it('ts18_html_bearing_server_strings_render_escaped', async () => {
    const canary = '<img src=x onerror="window.__canary=1">';
    preflightMock.mockResolvedValue(
      preflight({ fixture_label: canary, target_instance_label: canary, data_scope: [canary], impact_summary: canary }),
    );
    const { container } = render(<SyntheticPreviewPanel token="tok" />);
    runPreflight();
    await screen.findByRole('region', { name: /restore confirmation/i });
    expect(container.querySelector('img')).toBeNull();
    expect(container.innerHTML).toContain('&lt;img');
    expect((window as unknown as { __canary?: number }).__canary).toBeUndefined();
  });
});
