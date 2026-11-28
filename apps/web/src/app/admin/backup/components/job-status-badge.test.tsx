import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import JobStatusBadge from './job-status-badge';
import { KNOWN_BACKUP_JOB_STATUSES, type BackupJobStatusOrUnknown } from '@/domains/backup';

const LABELS: Record<BackupJobStatusOrUnknown, string> = {
  queued: 'Queued',
  running: 'Running',
  succeeded: 'Succeeded',
  failed: 'Failed',
  'recovery-required': 'Recovery required',
  incomplete: 'Incomplete',
  unknown: 'Unknown status',
};

describe('JobStatusBadge', () => {
  // TS-12a: exhaustive handling of every documented value plus the unknown fallback
  it.each([...KNOWN_BACKUP_JOB_STATUSES, 'unknown' as const])('ts12a_renders_%s_as_text_with_no_download_control', (status) => {
    const { container } = render(<JobStatusBadge status={status} />);
    expect(screen.getByText(LABELS[status])).toBeInTheDocument();
    expect(screen.queryByRole('button')).not.toBeInTheDocument();
    expect(screen.queryByRole('link')).not.toBeInTheDocument();
    expect(container.querySelector('a, [download]')).toBeNull();
  });

  // TS-37: status is conveyed by text, not colour or icon alone
  it('ts37_status_is_text_and_the_icon_is_hidden_from_assistive_technology', () => {
    const { container } = render(<JobStatusBadge status="failed" />);
    expect(screen.getByText('Failed')).toBeVisible();
    const svg = container.querySelector('svg');
    expect(svg).not.toBeNull();
    expect(svg).toHaveAttribute('aria-hidden', 'true');
  });

  it('labels an incomplete job as synthetic, database-only and not restorable when detailed', () => {
    render(<JobStatusBadge status="incomplete" detailed />);
    expect(screen.getByText('Incomplete')).toBeInTheDocument();
    expect(screen.getByText(/synthetic, database-only, not restorable/i)).toBeInTheDocument();
  });

  it('omits the incomplete description unless detailed', () => {
    render(<JobStatusBadge status="incomplete" />);
    expect(screen.queryByText(/not restorable/i)).not.toBeInTheDocument();
  });

  it('ts12a_unknown_status_shows_the_safe_fallback_and_never_a_success_label', () => {
    render(<JobStatusBadge status="unknown" />);
    expect(screen.getByText('Unknown status')).toBeInTheDocument();
    expect(screen.queryByText(/succeeded/i)).not.toBeInTheDocument();
  });
});
