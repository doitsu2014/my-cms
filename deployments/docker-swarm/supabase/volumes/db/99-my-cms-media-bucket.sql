-- 99-my-cms-media-bucket.sql
-- Idempotently seed the default public 'media' bucket used by my-cms.
-- Required because Supabase Storage hydrates buckets from the storage.buckets
-- table; without this row the local stack returns "Bucket not found" on every
-- object operation.

INSERT INTO storage.buckets (
    id,
    name,
    public,
    file_size_limit,
    allowed_mime_types,
    owner,
    type,
    created_at,
    updated_at
)
VALUES (
    'media',
    'media',
    true,
    52428800,
    NULL,
    '00000000-0000-0000-0000-000000000000',
    'STANDARD',
    now(),
    now()
)
ON CONFLICT (name) DO NOTHING;
