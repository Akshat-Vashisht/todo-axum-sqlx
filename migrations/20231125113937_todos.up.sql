-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS todos(
    todo_id uuid DEFAULT uuid_generate_v4(),
    todo_title VARCHAR(255) NOT NULL,
    todo_description VARCHAR(255) NOT NULL,
    created_at DATE DEFAULT CURRENT_DATE NOT NULL,
    PRIMARY KEY (todo_id)
)