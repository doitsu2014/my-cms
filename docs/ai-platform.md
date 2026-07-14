# AI Translation Platform

The AI Translation Platform is the CMS's AI augmentation layer. It turns multilingual content publishing into a background-friendly, cost-aware workflow: OpenAI produces high-quality translations while a vector similarity index (Qdrant or pgvector) lets the platform reuse previous translations instead of paying for duplicate work. The platform is exposed as a REST surface for the Rust API and surfaced in the React admin through a real-time job-tracking UI.

## Core Features

- **3-Tier Lookup Strategy** — Database → Qdrant/pgvector similarity → OpenAI. Minimizes API costs by short-circuiting on cache hits and high-similarity reuses before ever calling the model.
- **HTML-Aware Processing** — Preserves HTML structure when translating markup content; never splits a tag across chunks.
- **Smart Translation Reuse** — Automatically reuses highly similar translations (≥95% similarity) from previous posts in the same target language.
- **Background Processing** — Non-blocking execution with job tracking and progress monitoring for large posts.
- **Vector Database Integration** — Qdrant (and pgvector via Supabase) for semantic search and similarity matching over embeddings.
- **Model Selection** — Choose from multiple AI models (`gpt-5-nano`, `gpt-4o-mini`, `gpt-4o`) with cost transparency surfaced in the UI.

## Background Job Tracking

### Database Schema

- New `translation_jobs` table tracks the job lifecycle: `pending → processing → completed | failed`.
- Stores progress (0–100%), error messages, selected AI model, and timestamps.
- Indexed for efficient active-job queries.

### API Endpoints

- `POST /posts/{post_id}/translate/background` — Start a background translation job.
- `GET /posts/{post_id}/translate/jobs/{job_id}` — Get the status of a specific job.
- `GET /posts/{post_id}/translate/jobs` — Get all active jobs for a post.

### Frontend Features

- Real-time progress tracking with 2-second polling.
- Model selection dialog for re-translations.
- Active-job detection to prevent duplicate submissions.
- Automatic UI disabling when translations are in progress.
- Timeout handling (5 minutes) with user notifications.

### Job Status Response

```json
{
  "jobId": "uuid",
  "status": "processing",
  "progress": 45,
  "aiModel": "gpt-5-nano",
  "errorMessage": null,
  "createdAt": "2026-01-26T...",
  "updatedAt": "2026-01-26T..."
}
```

`status` is one of `pending`, `processing`, `completed`, or `failed`. `progress` ranges from 0 to 100.

## Implementation Details

For the deep dive — architecture flows, configuration, API reference, Qdrant collection layout, troubleshooting, and best practices — see the implementation document next to the code:

- [AI Translation implementation deep dive](apps/api/application_core/src/commands/ai/README.md)
- Source folder: [`apps/api/application_core/src/commands/ai/`](apps/api/application_core/src/commands/ai/)

## Related

- [Project root README](README.md) — project overview, architecture, quickstart, and the full documentation index.