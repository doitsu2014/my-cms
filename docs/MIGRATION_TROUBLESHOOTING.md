# Migration Troubleshooting Guide

## Failed Migration: m20260126_040610_release_300 (translation_jobs table)

If you encountered a syntax error when running the `m20260126_040610_release_300` migration, follow these steps to resolve it.

### Error Symptoms

```
syntax error at or near "("
position: Some(Original(493))
```

The error occurs when creating the `translation_jobs` table due to invalid SQL for non-unique indexes.

### Root Cause

The original migration code attempted to create a non-unique index inline with the CREATE TABLE statement, which is not supported in PostgreSQL. The inline `.index()` method generated:

```sql
CONSTRAINT "index_translation_jobs_post_id_status" ("post_id", "status")
```

This is invalid because it's a constraint without a type (UNIQUE, CHECK, etc.).

### Resolution Steps

#### Step 1: Deploy the Fixed Code

Ensure you have deployed the latest code from this PR that includes the fix.

#### Step 2: Check Migration Status

Connect to your PostgreSQL database and check if the migration was recorded:

```sql
SELECT * FROM seaql_migrations WHERE version = 'm20260126_040610_release_300';
```

#### Step 3: Clean Up Partial Migration

If the migration shows as applied but the table doesn't exist or is incomplete:

```sql
-- Remove the migration record
DELETE FROM seaql_migrations WHERE version = 'm20260126_040610_release_300';

-- Drop the table if it was partially created
DROP TABLE IF EXISTS translation_jobs CASCADE;

-- Drop the index if it exists separately
DROP INDEX IF EXISTS index_translation_jobs_post_id_status;
```

#### Step 4: Re-run Migration

With the fixed code deployed and the old migration cleaned up, re-run the migration:

```bash
# Using the CLI
cd services/migration
cargo run

# Or through the API endpoint (if available)
curl -X POST https://your-api-url/administrator/database/migration \
  -H "Authorization: Bearer YOUR_TOKEN"
```

#### Step 5: Verify Success

Check that the table and index were created correctly:

```sql
-- Check table structure
\d translation_jobs

-- Check indexes
\di index_translation_jobs_post_id_status
```

You should see:
- A `translation_jobs` table with all columns
- A primary key on `id`
- A foreign key to `posts(id)`
- An index `index_translation_jobs_post_id_status` on `(post_id, status)`

### Prevention

The fix ensures that:
1. Non-unique indexes are created as separate CREATE INDEX statements
2. The rollback (down migration) includes `.if_exists()` for safe cleanup
3. Future migrations will not encounter this issue

### Need Help?

If you continue to experience issues:
1. Check the migration logs for detailed error messages
2. Verify your PostgreSQL version is compatible (tested with PostgreSQL 15)
3. Ensure you have the necessary database permissions to create tables and indexes
