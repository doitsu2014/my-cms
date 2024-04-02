-- POSTGRES SQL Create Table for Posts
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    content TEXT NOT NULL,
    published   BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT NOT NULL,
    last_modified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_modified_by TEXT NOT NULL
)

