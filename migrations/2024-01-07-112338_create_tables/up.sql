CREATE TABLE IF NOT EXISTS teams (
    name    varchar(255)    primary key
);

CREATE TABLE IF NOT EXISTS players (
    nickname    varchar(255)    primary key,
    score       bigint          NOT NULL DEFAULT 0,
    team_name   varchar(255)    NOT NULL,
    CONSTRAINT fk_team FOREIGN KEY (team_name) REFERENCES teams (name) ON DELETE CASCADE
);
