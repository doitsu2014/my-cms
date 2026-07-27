interface EyebrowProps {
  children: React.ReactNode;
  withDot?: boolean;
}

const Eyebrow = ({ children, withDot = true }: EyebrowProps) => (
  <p className="eyebrow">
    {withDot && <span className="eyebrow__dot" aria-hidden="true" />}
    {children}
  </p>
);

export default Eyebrow;
