CREATE TABLE users (
    id VARCHAR(36) DEFAULT gen_random_uuid() PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(255) NOT NULL
);

INSERT INTO users (id, username, password)
VALUES
  ('17b5ac0c-1429-469a-8522-053f7bf0f80d', 'test', 'password');