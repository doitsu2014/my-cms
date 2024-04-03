-- POSTGRES SQL Create Table for Posts
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    content TEXT NOT NULL,
    published NOT NULL BOOLEAN DEFAULT FALSE,
    created_at NOT NULL TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT NOT NULL,
    last_modified_at NOT NULL TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_modified_by TEXT NOT NULL
)

