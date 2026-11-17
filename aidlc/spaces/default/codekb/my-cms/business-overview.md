# Business Overview

## Purpose
My-CMS is a content management and publishing platform with a Rust API, an administrative application, and a public reader. Its backend domains cover posts, authentication, media, users, and SEO. The backup/restore work adds operational protection for configured My-CMS/Supabase instances.

## Business Capabilities
- Authors and administrators manage posts and related content through the admin application.
- The public reader serves localized published content.
- Backend domains provide content, identity, media, user administration, and SEO capabilities.
- The backup/restore feature specification describes administrator-selected instance backup into a ZIP package and upload/preflight/explicitly confirmed restore to a target. This is target behavior, not implemented functionality.

## Users and Operational Context
Administrators manage content and protected system operations. Operators oversee deployment and recovery. Readers consume public content. Backup packages are sensitive data; package completeness, supported Supabase capabilities, consistency guarantees, retention, and restore safety require explicit verification.

## Evidence Boundary
This CodeKB is based on mixed-depth analysis. The gateway wiring, backup CLI, selected domain source trees, manifests, documentation, and workflow/configuration areas were examined at varying depth; frontend and deployment source areas were partly inventoried only. It is not an exhaustive file-by-file review. `docs/features/backup-restore.md` explicitly identifies a CLI skeleton and absence of a complete ZIP/UI/upload/restore flow. No tests, builds, or programs were run.
