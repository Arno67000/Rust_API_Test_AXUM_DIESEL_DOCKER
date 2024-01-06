CREATE TABLE IF NOT EXISTS users (
    id          bigserial       primary key,
    nickname    varchar(255)    NOT NULL UNIQUE,
    level       bigint          NOT NULL DEFAULT 0,
    score       bigint          NOT NULL DEFAULT 0,
    team_name   varchar(255)
);