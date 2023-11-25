-- Add up migration script here
CREATE TABLE credentials (
    id INT AUTO_INCREMENT,
    username VARCHAR(25) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

-- CREATE INDEX idx_username ON credentials (username);