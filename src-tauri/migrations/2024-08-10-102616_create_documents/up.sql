CREATE TABLE documents (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    path TEXT,
    metadata TEXT,
    imported_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    digested_at TIMESTAMP,
    digest_status TEXT CHECK(digest_status IN ('pending', 'in_progress', 'completed', 'failed')) DEFAULT 'pending',
    digest_error TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
