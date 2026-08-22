/** Validate a required server-only endpoint without exposing its value. */
export function requiredServerUrl(name, raw) {
  const value = raw?.trim();
  if (!value) throw new Error(`${name} is required`);
  try { new URL(value); } catch { throw new Error(`${name} is invalid`); }
  return value;
}
