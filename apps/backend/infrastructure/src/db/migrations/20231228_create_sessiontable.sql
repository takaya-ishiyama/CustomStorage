CREATE TABLE session (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) REFERENCES users(id),
    access_token VARCHAR(255) NOT NULL,
    refresh_token VARCHAR(255),
    expiration_timestamp TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE INDEX idx_session_user_id ON session(user_id);
CREATE INDEX idx_session_access_token ON session(access_token);