import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import InstanceSelector from './instance-selector';
import type { BackupInstance } from '@/domains/backup';

const instance: BackupInstance = {
  id: 'inst-1',
  label: 'Local synthetic instance',
  environment: 'development',
  operations: ['create'],
  capabilities: { create: true, upload: false, preflight: false, restore: false, download: false },
};

describe('InstanceSelector', () => {
  // TS-46
  it('ts46_renders_the_instance_label_and_environment_label', () => {
    render(<InstanceSelector instances={[instance]} state="ready" selectedId={null} onSelect={vi.fn()} />);
    expect(screen.getByRole('radio', { name: /Local synthetic instance/ })).toBeInTheDocument();
    expect(screen.getByText('Local synthetic instance')).toBeInTheDocument();
    expect(screen.getByText('development')).toBeInTheDocument();
  });

  it('ts46_marks_the_selected_instance_and_reports_selection', () => {
    const onSelect = vi.fn();
    const view = render(<InstanceSelector instances={[instance]} state="ready" selectedId={null} onSelect={onSelect} />);
    const option = screen.getByRole('radio', { name: /Local synthetic instance/ });
    expect(option).not.toBeChecked();
    fireEvent.click(option);
    expect(onSelect).toHaveBeenCalledWith('inst-1');
    view.rerender(<InstanceSelector instances={[instance]} state="ready" selectedId="inst-1" onSelect={onSelect} />);
    expect(screen.getByRole('radio', { name: /Local synthetic instance/ })).toBeChecked();
  });

  // TS-38
  it('ts38_empty_list_offers_nothing_to_select_and_says_so', () => {
    render(<InstanceSelector instances={[]} state="ready" selectedId={null} onSelect={vi.fn()} />);
    expect(screen.queryByRole('radio')).not.toBeInTheDocument();
    expect(screen.getByText(/no backup instance is available/i)).toBeInTheDocument();
  });

  it('ts38_unavailable_list_offers_nothing_to_select_and_a_retry', () => {
    const onRetry = vi.fn();
    render(<InstanceSelector instances={[]} state="error" selectedId={null} onSelect={vi.fn()} onRetry={onRetry} />);
    expect(screen.queryByRole('radio')).not.toBeInTheDocument();
    expect(screen.getByRole('alert')).toHaveTextContent(/unable to load/i);
    fireEvent.click(screen.getByRole('button', { name: /retry/i }));
    expect(onRetry).toHaveBeenCalledTimes(1);
  });

  it('ts38_loading_state_offers_nothing_to_select', () => {
    render(<InstanceSelector instances={[]} state="loading" selectedId={null} onSelect={vi.fn()} />);
    expect(screen.queryByRole('radio')).not.toBeInTheDocument();
    expect(screen.getByRole('status')).toBeInTheDocument();
  });

  it('renders server strings only as text', () => {
    const hostile = { ...instance, label: '<img src=x onerror=alert(1)>' };
    const { container } = render(<InstanceSelector instances={[hostile]} state="ready" selectedId={null} onSelect={vi.fn()} />);
    expect(container.querySelector('img')).toBeNull();
    expect(screen.getByText('<img src=x onerror=alert(1)>')).toBeInTheDocument();
  });
});
