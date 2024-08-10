CREATE TABLE conversations (
    id INTEGER PRIMARY KEY,
    agent_id TEXT NOT NULL,
    title TEXT,
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP,
    status TEXT CHECK(status IN ('active', 'completed', 'abandoned')) DEFAULT 'active',
    FOREIGN KEY (agent_id) REFERENCES agents (id)
);
