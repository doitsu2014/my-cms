# consolidate-category-ai-translate-into-domain-posts

Fold the category CRUD, AI model registry, and post translation pipeline into domain_posts. This consolidates post-related concerns that were deferred from the original pluggable-domain refactor and avoids creating extra domain crates for capabilities that are integral to the post vertical slice. Categories remain a sub-aggregate owned by posts (foreign-keyed from posts.category_id) and translation remains a post-pipeline capability that writes through the existing domain_posts entities.
