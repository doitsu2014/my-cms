-- volumes/db/99-my-cms-grants.sql
-- my-cms specific grants layered on top of the upstream Supabase init.
-- Mounted at /docker-entrypoint-initdb.d/migrations/99-my-cms-grants.sql
-- All statements are idempotent and safe to re-run.

-- Defensive: ensure supabase_storage_admin can CREATE the storage schema
-- during its tenant migration. The supabase/postgres image's init should
-- grant this, but the storage-api v1.60.x migration emits SQLSTATE 42501
-- (aclcheck_error) without it. Idempotent.
GRANT CREATE ON DATABASE postgres TO supabase_storage_admin;

-- Defensive: ensure pgvector is available for the my-cms AI translation
-- feature. The supabase/postgres image ships with it, but we add this to
-- guard against image downgrades.
CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA public;
