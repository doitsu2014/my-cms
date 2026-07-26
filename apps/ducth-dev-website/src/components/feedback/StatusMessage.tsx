interface StatusMessageProps {
  children: React.ReactNode;
  className?: string;
}

const StatusMessage = ({ children, className = '' }: StatusMessageProps) => (
  <div className={`status-message ${className}`.trim()} role="status" aria-live="polite">
    {children}
  </div>
);

export default StatusMessage;
