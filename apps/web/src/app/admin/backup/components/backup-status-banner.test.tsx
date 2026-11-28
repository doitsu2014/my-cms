import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import BackupStatusBanner from './backup-status-banner';
import { backupStatusSchema, type BackupStatus } from '@/domains/backup';

const ready: BackupStatus = {
  state: 'ready',
  disabled_reason: null,
  db_healthy: true,
  admissions_open: true,
  synthetic_only: true,
  reconciliation: { last_completed_at: '2026-11-22T10:15:00Z', last_result: 'ok' },
  cleanup: { pending: 0, exhausted: 0, orphan_discovered: 0, ownership_incidents: 0 },
  escalation: { owner: 'project_owner', visible: false },
};

const disabled = (reason: BackupStatus['disabled_reason']): BackupStatus => ({
  ...ready,
  state: 'disabled',
  disabled_reason: reason,
  admissions_open: false,
  reconciliation: null,
  cleanup: null,
  escalation: { owner: 'project_owner', visible: true },
});

describe('BackupStatusBanner', () => {
  // TS-50: escalation
  it('ts50_escalation_visible_names_the_project_owner_and_cleanup_exhaustion', () => {
    const status: BackupStatus = {
      ...ready,
      state: 'degraded',
      cleanup: { pending: 0, exhausted: 2, orphan_discovered: 0, ownership_incidents: 0 },
      escalation: { owner: 'project_owner', visible: true },
    };
    render(<BackupStatusBanner status={status} phase="ready" />);
    const banner = screen.getByRole('alert');
    expect(banner).toHaveTextContent(/degraded/i);
    expect(banner).toHaveTextContent(/escalate to the project owner/i);
    expect(banner).toHaveTextContent(/2 cleanup/i);
  });

  it('ts50_escalation_visible_for_ownership_incidents_alone', () => {
    const status: BackupStatus = {
      ...ready,
      cleanup: { pending: 0, exhausted: 0, orphan_discovered: 0, ownership_incidents: 1 },
      escalation: { owner: 'project_owner', visible: true },
    };
    render(<BackupStatusBanner status={status} phase="ready" />);
    expect(screen.getByRole('alert')).toHaveTextContent(/project owner/i);
    expect(screen.getByRole('alert')).toHaveTextContent(/1 ownership/i);
  });

  it('ts50_ready_without_escalation_shows_a_status_and_no_escalation_text', () => {
    render(<BackupStatusBanner status={ready} phase="ready" />);
    expect(screen.getByRole('status')).toHaveTextContent(/ready/i);
    expect(screen.getByRole('status')).toHaveTextContent(/synthetic/i);
    expect(screen.queryByText(/project owner/i)).not.toBeInTheDocument();
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
  });

  it('ts50_not_ready_state_is_announced_without_escalation', () => {
    render(<BackupStatusBanner status={{ ...ready, state: 'not_ready', admissions_open: false, reconciliation: null, cleanup: null }} phase="ready" />);
    expect(screen.getByRole('status')).toHaveTextContent(/not ready/i);
    expect(screen.queryByText(/project owner/i)).not.toBeInTheDocument();
  });

  it('ts50_database_unhealthy_is_stated_in_text', () => {
    render(<BackupStatusBanner status={{ ...ready, state: 'degraded', db_healthy: false, reconciliation: null, cleanup: null, escalation: { owner: 'project_owner', visible: true } }} phase="ready" />);
    expect(screen.getByRole('alert')).toHaveTextContent(/database is unavailable/i);
    expect(screen.getByRole('alert')).toHaveTextContent(/project owner/i);
  });

  it.each([
    ['NOT_CONFIGURED', /not configured/i],
    ['NON_LOOPBACK_BIND', /loopback/i],
    ['INVALID_CONFIG', /configuration is invalid/i],
  ] as const)('ts50_disabled_with_reason_%s_states_the_reason_and_names_the_project_owner', (reason, pattern) => {
    render(<BackupStatusBanner status={disabled(reason)} phase="ready" />);
    const banner = screen.getByRole('alert');
    expect(banner).toHaveTextContent(/backup is disabled/i);
    expect(banner).toHaveTextContent(pattern);
    expect(banner).toHaveTextContent(/project owner/i);
  });

  it('ts50_disabled_with_a_null_reason_still_renders_a_generic_disabled_message', () => {
    render(<BackupStatusBanner status={disabled(null)} phase="ready" />);
    expect(screen.getByRole('alert')).toHaveTextContent(/backup is disabled/i);
  });

  it('ts50_schema_rejects_a_disabled_reason_outside_the_closed_set_so_the_banner_never_sees_it', () => {
    expect(backupStatusSchema.safeParse({ ...disabled('NOT_CONFIGURED'), disabled_reason: 'SECRET_PATH' }).success).toBe(false);
    for (const reason of ['NOT_CONFIGURED', 'NON_LOOPBACK_BIND', 'INVALID_CONFIG']) {
      expect(backupStatusSchema.safeParse({ ...disabled('NOT_CONFIGURED'), disabled_reason: reason }).success).toBe(true);
    }
  });

  it('shows fixed copy while loading and when the status request failed', () => {
    const { rerender } = render(<BackupStatusBanner status={null} phase="loading" />);
    expect(screen.getByRole('status')).toHaveTextContent(/checking backup status/i);
    rerender(<BackupStatusBanner status={null} phase="error" />);
    expect(screen.getByRole('alert')).toHaveTextContent(/backup status is unavailable/i);
  });

  // TS-18
  it('ts18_html_bearing_owner_string_renders_escaped', () => {
    const canary = '<img src=x onerror="window.__canary=1">';
    const status: BackupStatus = { ...ready, state: 'degraded', escalation: { owner: canary, visible: true } };
    const { container } = render(<BackupStatusBanner status={status} phase="ready" />);
    expect(container.querySelector('img')).toBeNull();
    expect(container.innerHTML).toContain('&lt;img');
    expect((window as unknown as { __canary?: number }).__canary).toBeUndefined();
  });
});
