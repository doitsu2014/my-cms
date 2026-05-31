-- volumes/db/init/00-setup-roles.sql
-- Creates required roles for Supabase services on first startup.
-- WARNING: The password below must match POSTGRES_PASSWORD in .env

-- Used by GoTrue auth service
CREATE ROLE supabase_auth_admin WITH LOGIN CREATEROLE NOINHERIT PASSWORD 'change-me-to-a-secure-password';

-- Used by Storage API
CREATE ROLE supabase_storage_admin WITH LOGIN CREATEROLE NOINHERIT PASSWORD 'change-me-to-a-secure-password';

-- Used by PostgREST for anonymous requests
CREATE ROLE anon WITH LOGIN NOINHERIT;
CREATE ROLE authenticated WITH LOGIN NOINHERIT;
CREATE ROLE authenticator WITH LOGIN NOINHERIT PASSWORD 'change-me-to-a-secure-password';

-- Used by service_role key
CREATE ROLE service_role WITH LOGIN NOINHERIT;

-- Used by Supavisor connection pooler
CREATE ROLE supabase_admin WITH LOGIN CREATEROLE NOINHERIT PASSWORD 'change-me-to-a-secure-password';

-- Grant schema access to authenticator
GRANT anon, authenticated, service_role TO authenticator;

-- Grant necessary permissions to supabase_auth_admin
GRANT CREATE ON DATABASE postgres TO supabase_auth_admin;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO supabase_auth_admin;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO supabase_auth_admin;

-- Grant necessary permissions to supabase_storage_admin
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO supabase_storage_admin;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO supabase_storage_admin;

-- Grant anonymous role access
GRANT USAGE ON SCHEMA public TO anon, authenticated, service_role;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO anon, authenticated, service_role;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO anon, authenticated, service_role;

-- Enable pgvector for AI translation (replaces Qdrant)
CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA public;

-- Enable other useful extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;
CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;
