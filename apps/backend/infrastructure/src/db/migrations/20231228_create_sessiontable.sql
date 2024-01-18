CREATE TABLE session (
    user_id uuid REFERENCES users(id) PRIMARY KEY,
    access_token VARCHAR(255) NOT NULL,
    refresh_token VARCHAR(255),
    expiration_timestamp TIMESTAMP NOT NULL,
    expiration_timestamp_for_refresh TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);


CREATE INDEX idx_session_user_id ON session(user_id);
CREATE INDEX idx_session_access_token ON session(access_token);