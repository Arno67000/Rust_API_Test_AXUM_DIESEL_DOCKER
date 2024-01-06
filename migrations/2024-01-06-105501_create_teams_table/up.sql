CREATE TABLE IF NOT EXISTS teams (
    id          bigserial       primary key,
    name    varchar(255)    NOT NULL UNIQUE
);