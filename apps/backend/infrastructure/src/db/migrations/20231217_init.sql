CREATE TABLE users (
    -- id VARCHAR(36) DEFAULT gen_random_uuid() PRIMARY KEY,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP

);

INSERT INTO users (id, username, password)
VALUES
  ('17b5ac0c-1429-469a-8522-053f7bf0f80d', 'test', 'password');