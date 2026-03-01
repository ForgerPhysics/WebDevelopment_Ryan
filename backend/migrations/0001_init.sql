CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR UNIQUE,
    password_hash VARCHAR,
    created_at TIMESTAMP
);

CREATE TABLE forms (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    title VARCHAR,
    description TEXT,
    created_at TIMESTAMP
);
