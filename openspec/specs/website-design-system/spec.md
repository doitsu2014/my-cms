# website-design-system Specification

## Purpose
TBD - created by archiving change redesign-ducth-dev-website. Update Purpose after archive.
## Requirements
### Requirement: Ink & Tide color tokens

The system SHALL expose a single `ink-tide` theme as the active theme on every reader route. The theme SHALL define the following CSS custom properties on the root element: a parchment page background, a fresh-paper surface, a deep ink foreground, a muted ink foreground, a hairline border color, a cinnabar accent, an accent wash, a deep ink ground, and a success/warning/error trio reserved for functional feedback. The theme SHALL NOT define emerald or any rotating semantic category color.

#### Scenario: Active theme is ink-tide

- **WHEN** any reader route renders
- **THEN** the root element has `data-theme="ink-tide"` and the parchment background color is applied as the page background

#### Scenario: No emerald theme reaches production

- **WHEN** the SSR HTML is inspected
- **THEN** no element has `data-theme="emerald"` and no CSS rule defines the emerald palette

### Requirement: Light and dark parity

The theme SHALL provide an explicit light variant and an explicit dark variant with the same semantic roles. Each role SHALL satisfy at least 4.5:1 contrast against its paired background for body text and 3:1 for large text. The dark variant SHALL darken the page background and lighten the foreground while keeping the cinnabar accent visually distinguishable.

#### Scenario: Dark variant has parity

- **WHEN** the user toggles dark mode
- **THEN** the page background, foreground, hairline, and accent swap to the dark variant values and no element renders as the light variant

#### Scenario: Cinnabar accent remains distinguishable in dark mode

- **WHEN** dark mode is active and a cinnabar accent is rendered on the dark ink ground
- **THEN** the accent is visible with at least 3:1 contrast against the dark background

### Requirement: Typography scale

The system SHALL load Noto Serif Display for display type, Inter for body and UI, and JetBrains Mono for code. The scale SHALL define fluid display sizes for XL, L, and H1 using `clamp(...)`; fixed sizes for H2, H3, eyebrow, and body; and a 68ch maximum measure for article prose. Text SHALL NOT be justified.

#### Scenario: Display size scales fluidly

- **WHEN** the viewport width changes between 360px and 1440px
- **THEN** the display heading size stays within the configured clamp range and never overlaps the section padding

#### Scenario: Article body is constrained to 68ch

- **WHEN** a post page renders the article body
- **THEN** the body's computed width is at most 68ch and the paragraphs are left-aligned without justification

### Requirement: Spacing and container primitives

The system SHALL define a spacing scale of `4, 8, 12, 16, 24, 32, 48, 64, 96, 128px` and a `Container` primitive with a 1240px maximum width and a fluid gutter between `clamp(20px, 4vw, 48px)`. Pages SHALL compose full-width sections with internal containers rather than nesting a constrained `<main>`.

#### Scenario: Container caps at 1240px

- **WHEN** the viewport width is 1920px
- **THEN** the container's computed width is at most 1240px and the outer whitespace grows symmetrically

#### Scenario: Sections render full-width

- **WHEN** a page renders a section with a hairline background
- **THEN** the section spans the entire viewport width and only the inner content is constrained by the container

### Requirement: Motion contract

The system SHALL expose a motion contract that defines hover and active transitions at 160ms ease-out, padding transitions at 200ms, and entrance animations at 240–320ms with 8px upward movement. Entrance animations SHALL NOT be applied to every list row. The system SHALL respect `prefers-reduced-motion: reduce` by removing entrance translations, smooth scrolling, and hover transforms while keeping instantaneous state changes visible.

#### Scenario: Reduced motion disables entrance animations

- **WHEN** the user has `prefers-reduced-motion: reduce` set
- **THEN** no element animates on entry and the page is fully readable without motion

#### Scenario: List rows do not animate on entry

- **WHEN** the home page renders the recent articles list
- **THEN** the rows render in their final position without staggered entrance animation

### Requirement: Focus and prose styles

The system SHALL define a single visible focus ring (2px outline, 3px offset) used by every interactive element. The system SHALL define a `.article-prose` contract that styles headings, paragraphs, links, blockquotes, lists, inline code, code blocks, separators, pull quotes, and images within the article body. The prose contract SHALL NOT depend on Tailwind Typography.

#### Scenario: Focus ring is visible and consistent

- **WHEN** any interactive element receives keyboard focus
- **THEN** it shows a 2px outline with 3px offset using the accent color

#### Scenario: Article prose renders TipTap output

- **WHEN** a post page renders a TipTap document
- **THEN** headings, paragraphs, lists, code blocks, and blockquotes use the prose contract styles and the existing Highlight.js code highlighting is preserved

### Requirement: No new third-party icon library

The system SHALL NOT add a new icon library dependency. Inline SVGs already present in the supplied design assets SHALL be reused, and any missing icon SHALL be rendered as text or a simple inline SVG.

#### Scenario: No icon library is added

- **WHEN** the package.json is inspected
- **THEN** `lucide-react`, `react-icons`, `@heroicons/react`, and any other icon package are not present

