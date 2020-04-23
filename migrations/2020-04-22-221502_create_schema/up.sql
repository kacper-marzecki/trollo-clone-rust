-- Your SQL goes here


CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id         uuid default uuid_generate_v4() PRIMARY KEY,
    username   VARCHAR(64)  NOT NULL,
    email      VARCHAR(100) NOT NULL,
    password   VARCHAR(64)  NOT NULL,
    avatar_id  INT          NULL,
    created_at TIMESTAMP    NOT NULL
);
CREATE INDEX users_email_idx ON users (email);
CREATE INDEX users_username_idx ON users (username);


create table card_task_items
(
    id          uuid primary key,
    card_id     uuid,
    text_       text,
    is_complete bool
);

