import { useState } from 'react';

export const getRandomColor = () => {
  const colors = [
    'bg-primary',
    'bg-secondary',
    'bg-accent',
    'bg-neutral',
    'bg-base-200',
    'bg-base-300',
    'bg-info',
    'bg-success',
    'bg-warning',
    'bg-error'
  ];
  return colors[Math.floor(Math.random() * colors.length)];
};

export default function MultiChipInput({
  chips,
  setChips,
  className,
  loading,
  formControlName
}: {
  chips: { label: string; color: string }[];
  setChips: (chips: { label: string; color: string }[]) => void;
  className?: string;
  loading: boolean;
  formControlName: string;
}) {
  const [inputValue, setInputValue] = useState('');

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if ((event.key === 'Enter' || event.key === ',') && !loading) {
      event.preventDefault();
      const newChips = inputValue
        .split(',')
        .map((chip) => chip.trim())
        .filter((chip) => chip !== '' && !chips.some((c) => c.label === chip))
        .map((chip) => ({ label: chip, color: getRandomColor() }));

      setChips([...chips, ...newChips]);
      setInputValue('');
    }
  };

  const removeChip = (chipToRemove: string) => {
    if (!loading) {
      const afterRemove = chips.filter((chip) => chip.label !== chipToRemove);
      setChips(afterRemove);
    }
  };

  return (
    <div className={className}>
      {chips.map((chip, index) => (
        <div
          key={index}
          className={`inline-flex items-center px-3 py-1 m-1 rounded-full text-white ${chip.color} hover:opacity-90`}
          title={chip.label}>
          <span className="mr-2 truncate max-w-xs">{chip.label}</span>
          <button
            className="text-white hover:text-gray-300 font-bold"
            onClick={() => removeChip(chip.label)}
            disabled={loading}
            type="button">
            &times;
          </button>
        </div>
      ))}
      <input
        type="text"
        placeholder="Enter items..."
        className="input input-bordered w-full"
        value={inputValue}
        onChange={(e) => setInputValue(e.target.value)}
        onKeyDown={handleKeyDown}
        disabled={loading}
      />
      <input type="hidden" name={formControlName} value={JSON.stringify(chips)} />
    </div>
  );
}
