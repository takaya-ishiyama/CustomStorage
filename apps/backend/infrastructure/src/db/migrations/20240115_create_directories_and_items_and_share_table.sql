CREATE TABLE directories (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid REFERENCES users(id) NOT NULL,
    directories_id uuid REFERENCES directories(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
    CHECK (id <> directories_id)
);

CREATE TABLE items (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    directories_id uuid REFERENCES directories(id) NOT NULL,
    texts VARCHAR(1000),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE share (
    id SERIAL PRIMARY KEY,
    items_id uuid REFERENCES items(id) NOT NULL,
    user_id uuid REFERENCES users(id),
    publish boolean,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);