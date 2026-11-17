# Project-Level Rules

> Project-specific specialisation and corrections. Loaded after `org.md` and
> `team.md` as strict-additive guidance; contradictions with broader policy
> are rejected. Populated by practices-discovery and the self-learning loop.
>
> Use sparingly: most teams don't need a project layer. Reach for it
> only when this specific project needs stable, durable guidance beyond the
> team practice (for example, package-specific release checks or an additional
> regression suite for a legacy component).

## Way of Working

<!-- Project-specific specialisation. Example: -->
<!-- This monorepo requires package-scoped branch names and a package owner -->
<!-- review in addition to the team's normal merge policy. -->

## Walking Skeleton

<!-- Project-specific specialisation. Example: -->
<!-- The walking skeleton must exercise the legacy service adapter as well -->
<!-- as the new service boundary. -->

## Testing Posture

<!-- Project-specific specialisation. -->

## Guard Policy

<!-- Project-specific. Mode: strict, relaxed, or off. Strict here holds for every intent, whatever one person asks in chat; relaxed or off here applies to every intent whose policy came from its scope, unless the person sets another; changing this line changes it. A section under the retired Change Control heading, written by an earlier release, is still read. -->

## Deployment

<!-- Project-specific specialisation. -->

## Code Style

<!-- Project-specific specialisation. -->

## Tech Stack

Rust Axum/SeaORM gateway plus domain crates; React admin (`apps/web`) and
public reader (`apps/ducth-dev-website`); Supabase Postgres, Storage, and
GoTrue; OpenAI for translation and embeddings. Claude Code with AI-DLC is
the only development harness.

## Decided

DECIDED: Project requirements live in `aidlc/spaces/default/knowledge/aidlc-shared/`. Do not recreate OpenSpec or non-Claude harness configs. (migration, 2026-11-15)

DECIDED: The shipped API binary is gateway `my-cms-api`. Do not restore `application_core`, a standalone posts binary, or a separate migration crate. Tag helpers stay in `domain_posts`. (migration, 2026-11-15)

## Scope Overrides

<!-- Custom scope rules for this project. -->

## Forbidden

<!-- Populated by practices-discovery affirmation gate. -->
<!-- Format: NEVER [behavior] (affirmed [date]) -->
<!-- Example: NEVER throw exceptions across service layer boundaries (affirmed 2026-05-17) -->

None. (affirmed 2026-11-17)

> Không có câu trả lời nào nêu một ràng buộc cứng ở dạng điều cấm hoặc bắt buộc độc lập ngoài các thực hành được ghi nhận trong `team-practices.md`; vì vậy không chuyển các thực hành đó thành quy tắc `ALWAYS`/`NEVER`. (affirmed 2026-11-17)

## Mandated

<!-- Populated by practices-discovery affirmation gate. -->
<!-- Format: ALWAYS [behavior] (affirmed [date]) -->
<!-- Example: ALWAYS use Result<T,E> for fallible operations in service layer (affirmed 2026-05-17) -->

None. (affirmed 2026-11-17)

## Corrections

<!-- Project-specific corrections from human feedback. -->
<!-- Format: NEVER/ALWAYS [behavior] (learned [date]) -->
