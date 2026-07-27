interface PullQuoteProps {
  children: React.ReactNode;
  cite?: React.ReactNode;
}

const PullQuote = ({ children, cite }: PullQuoteProps) => (
  <blockquote className="pull-quote">
    <p>{children}</p>
    {cite && <cite>{cite}</cite>}
  </blockquote>
);

export default PullQuote;
