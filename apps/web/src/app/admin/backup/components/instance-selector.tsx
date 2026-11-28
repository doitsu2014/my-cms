import type { BackupInstance } from '@/domains/backup';

export interface InstanceSelectorProps {
  instances: BackupInstance[];
  state: 'loading' | 'ready' | 'error';
  selectedId: string | null;
  onSelect: (id: string) => void;
  onRetry?: () => void;
}

/**
 * F-43: lists the eligible backup instances (label and environment). An empty, loading or failed list offers
 * nothing to select, so the page keeps submission disabled. Server strings render as React text only.
 */
export default function InstanceSelector({ instances, state, selectedId, onSelect, onRetry }: InstanceSelectorProps) {
  if (state === 'loading') {
    return <div role="status" className="loading loading-spinner" aria-label="Loading backup instances" />;
  }

  if (state === 'error') {
    return (
      <div role="alert" className="alert alert-error">
        <span>Unable to load backup instances.</span>
        {onRetry && (
          <button type="button" className="btn btn-sm" onClick={onRetry}>
            Retry
          </button>
        )}
      </div>
    );
  }

  if (instances.length === 0) {
    return (
      <div className="alert alert-warning">
        <span>No backup instance is available.</span>
      </div>
    );
  }

  return (
    <fieldset className="space-y-2">
      <legend className="font-semibold mb-2">Instance</legend>
      {instances.map((instance) => (
        <label key={instance.id} className="flex items-center gap-3 cursor-pointer">
          <input
            type="radio"
            name="backup-instance"
            className="radio radio-primary"
            value={instance.id}
            checked={selectedId === instance.id}
            onChange={() => onSelect(instance.id)}
          />
          <span className="font-medium">{instance.label}</span>
          <span className="badge badge-outline">{instance.environment}</span>
        </label>
      ))}
    </fieldset>
  );
}
