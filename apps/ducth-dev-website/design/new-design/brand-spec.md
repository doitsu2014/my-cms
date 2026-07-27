# Ink & Tide — Design System

A bilingual editorial system for Duc Tran's writing, rooted in **Triết học phương đông** (Eastern philosophy) and the author's **Nhâm thuỷ** (Ren Water, Yang Water) element. The aesthetic is ink-wash + slow current: warm parchment surfaces, deep ink-blue type, one decisive cinnabar accent. The site reads like an editor's notebook beside a wide river — considered, generous, and unhurried.

## One-sentence system

**Editorial minimalism in the ink-wash palette of an Eastern study, with vermilion seal-red as the only accent that interrupts the page.**

## Hue anchor

The `--fg` / `--ink-deep` hues (255–260) are deliberately chosen to sit in the same hue family as the **architecture photo's blue sky** (~`oklch(60% 0.15 250)`). When the photo lands on a page — as the Systems category intro seal or the home closing CTA backdrop — its sky tonally fuses with the dark ground beneath it. Do not drift `--fg` or `--ink-deep` to a different hue family; the blue anchor is the seam that lets real photography and the design system share the same page.

## Tokens

All colors expressed in `oklch()` for perceptual uniformity; `oklch(L C H)`.

### Light theme (refined v2 — warmer, deeper, more "ink-wash")

| Token | OKLCH | Hex equivalent | Role |
|---|---|---|---|
| `--bg` | `oklch(92% 0.032 70)` | `#ece1cb` | Page surface — warm parchment (aged paper) |
| `--surface` | `oklch(97% 0.015 70)` | `#f6efde` | Elevated surface — fresh paper |
| `--fg` | `oklch(18% 0.055 260)` | `#0e1530` | Foreground — deep ink (cool-blue anchor) |
| `--muted` | `oklch(42% 0.022 260)` | `#5d6479` | Secondary text, captions |
| `--border` | `oklch(82% 0.022 78)` | `#d4caad` | Hairline divider, card edge |
| `--accent` | `oklch(50% 0.22 28)` | `#b13a25` | Vermilion cinnabar (朱) — one accent only |
| `--accent-ink` | `oklch(95% 0.05 30)` | `#fbe6da` | Accent wash background |
| `--ink-deep` | `oklch(12% 0.055 260)` | `#0a112a` | Display headings, ink ground |
| `--success` | `oklch(46% 0.14 155)` | `#2a8257` | Success (jade green) |
| `--warn` | `oklch(72% 0.15 72)` | `#d09c3a` | Warning (ochre) |
| `--danger` | `oklch(54% 0.22 25)` | `#b8311c` | Danger (deep cinnabar) |

### Dark theme (parity, refined v2)

| Token | OKLCH | Role |
|---|---|---|
| `--bg` | `oklch(13% 0.028 260)` | Ink-night surface (slightly deeper) |
| `--surface` | `oklch(17% 0.032 260)` | Elevated |
| `--fg` | `oklch(95% 0.018 85)` | Bone-white text (slightly warmer) |
| `--muted` | `oklch(68% 0.015 252)` | Muted |
| `--border` | `oklch(28% 0.022 252)` | Hairline |
| `--accent` | `oklch(66% 0.19 28)` | Vermilion (lighter for dark) |
| `--accent-ink` | `oklch(25% 0.075 28)` | Accent wash |
| `--ink-deep` | `oklch(98% 0.012 85)` | Display inverted |

## Typography

| Role | Family | Weights | Source |
|---|---|---|---|
| Display | **Noto Serif Display** | 600 / 700 / 800 | Google Fonts; full Vietnamese + Latin |
| Body | **Inter** | 400 / 500 / 600 | Google Fonts; full Vietnamese + Latin |
| Mono | **JetBrains Mono** | 400 / 600 | Google Fonts; code only |

Display face ≠ body face. Two families. Mono restricted to code blocks.

### Scale (multiplicative 1.25)

| Role | Size | Line-height | Tracking |
|---|---:|---:|---|
| Display XL | `clamp(48px, 7vw, 96px)` | 1.05 | `-0.025em` |
| Display L | `clamp(36px, 5vw, 64px)` | 1.08 | `-0.02em` |
| H1 | `clamp(32px, 4vw, 48px)` | 1.12 | `-0.018em` |
| H2 | `28px` | 1.18 | `-0.01em` |
| H3 | `22px` | 1.25 | `-0.005em` |
| Body | `17px` | 1.65 | `0` |
| Small | `14px` | 1.5 | `0.01em` |
| Eyebrow | `12px` | 1.4 | `0.18em` (ALL CAPS — `0.18em` tracking is intentional and high; required for caps legibility per typography craft) |

Vietnamese diacritics accommodated via line-height ≥ 1.6 on body. Webfont fallback: serif → `Georgia, "Times New Roman", serif`; sans → `system-ui, -apple-system, "Helvetica Neue", Arial, sans-serif`.

## Posture rules

1. **Ink-wash hierarchy** — Display sits in deep, breathing space; body occupies a measured 65ch column. The bigger the type, the more whitespace.
2. **One vermilion accent per screen** — Cinnabar marks exactly one editorial element (chapter mark, primary CTA, or pull-quote accent). Never two; never decorative.
3. **Asymmetric editorial grid** — Primary content on 2/3, secondary on 1/3; featured images span asymmetric widths. No centered-everything layouts.
4. **Vietnamese diacritic spacing** — line-height 1.65 on body, generous letter-spacing on small caps, no `text-align: justify`.
5. **Quiet motion only** — fade + 8px translate-up on entry (160ms ease-out). No parallax, no bouncing icons, no hero animations.
6. **Two-weight system** — Display 600 / 700, body 400 / 500, UI 500 / 600. No 800 except the home hero. No `font-weight: bold` defaults.
7. **Real images only** — featured visuals are SVG ink motifs (intentional art, not fake photos). If real photography becomes available, drop in via `<figure>` — never as a background fill.

## Component contracts

- **Header**: thin hairline border bottom, wordmark + nav + language toggle. No drop shadow.
- **Hero**: editorial pull-quote style — display title left-aligned, subtitle right-aligned (or below on mobile). No button.
- **Article card**: image-on-top variant with overlay-free design (image edge → metadata → title). No left-border accent. No emoji icons.
- **Eyebrow chip**: small caps, `--muted`, tracked `0.18em`, no background.
- **Primary button**: ink-deep ground, surface text, 4px radius, padding 12×24, hover → accent ground. Secondary → outlined ink-deep on transparent.
- **Pull-quote**: oversized serif at 32–40px, vermilion vertical bar (3px) on the left, italic optional.
- **Pagination**: text-only, numbered, current page in ink-deep.
- **Footer**: hairline top border, three columns (about / navigation / language), quiet.

## Accent discipline

- `--accent` visible at most **twice per screen**: once as a CTA / eyebrow mark, once as a small graphic flourish.
- Hover/focus rings count toward the budget.
- No "rainbow" of semantic colors. Status uses muted variants of `--success` / `--warn` / `--danger` only where functionally required (form errors).

## Anti-patterns

- No indigo / purple / "trust" gradients.
- No emoji as icons.
- No rounded card + colored left-border combo.
- No `font-family: system-ui` alone on headings.
- No `text-align: justify` on body copy.
- No `white-space: nowrap` on display type that could push past container width.
- No invented metrics, no `lorem ipsum`, no `feature one / two / three` placeholders.
