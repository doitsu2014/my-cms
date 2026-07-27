export function getArtworkVariant(slug: string): number {
  return Array.from(slug).reduce((sum, character) => sum + character.charCodeAt(0), 0) % 4;
}
