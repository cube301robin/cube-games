CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE teams (
    id         SERIAL PRIMARY KEY,
    uuid       UUID NOT NULL DEFAULT uuid_generate_v4(),
    name       TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE users (
    id                        SERIAL PRIMARY KEY,
    uuid                      UUID NOT NULL DEFAULT uuid_generate_v4(),
    name                      TEXT NOT NULL,
    email                     TEXT UNIQUE,
    team_id                   INT REFERENCES teams(id) ON DELETE SET NULL,
    strava_athlete_id         BIGINT UNIQUE,
    strava_access_token       TEXT,
    strava_refresh_token      TEXT,
    strava_token_expires_at   TIMESTAMPTZ,
    is_admin                  BOOLEAN NOT NULL DEFAULT false,
    created_at                TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_users_team_id   ON users(team_id);
CREATE INDEX idx_users_strava_id ON users(strava_athlete_id) WHERE strava_athlete_id IS NOT NULL;

CREATE TABLE games (
    id          TEXT PRIMARY KEY,   -- 'walk_run_2026', 'quiz_battle', etc.
    name        TEXT NOT NULL,
    description TEXT,
    active      BOOLEAN NOT NULL DEFAULT true,
    start_date  DATE,
    end_date    DATE,
    config      JSONB NOT NULL DEFAULT '{}',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Seed the walk_run_2026 game
INSERT INTO games (id, name, description, active, config)
VALUES (
  'walk_run_2026',
  'Walk & Run Edition',
  'Track your running and walking distance on Strava. 1 km = 1 point plus tier bonuses.',
  true,
  '{"activity_types": ["Run", "Walk"], "club_url": "https://www.strava.com/clubs/cubegames2026"}'
);

CREATE TABLE activities (
    id            BIGSERIAL PRIMARY KEY,
    external_id   TEXT NOT NULL,          -- strava activity id or quiz submission id
    source        TEXT NOT NULL,          -- 'strava', 'manual', 'quiz_api'
    user_id       INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    game_id       TEXT NOT NULL REFERENCES games(id),
    activity_type TEXT NOT NULL,          -- 'Run', 'Walk', 'QuizAnswer'
    value         DOUBLE PRECISION NOT NULL, -- km for strava, raw points for quiz
    occurred_at   TIMESTAMPTZ NOT NULL,
    raw_data      JSONB,
    inserted_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(source, external_id)
);
CREATE INDEX idx_activities_lookup ON activities(user_id, game_id, occurred_at);
CREATE INDEX idx_activities_week   ON activities(game_id, occurred_at);

CREATE TABLE weekly_individual_scores (
    id           BIGSERIAL PRIMARY KEY,
    user_id      INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id      INT REFERENCES teams(id),
    game_id      TEXT NOT NULL REFERENCES games(id),
    year         INT NOT NULL,
    iso_week     INT NOT NULL,
    raw_value    DOUBLE PRECISION NOT NULL,
    tier_name    TEXT,
    base_points  DOUBLE PRECISION NOT NULL,
    bonus_points DOUBLE PRECISION NOT NULL,
    total_points DOUBLE PRECISION NOT NULL,
    computed_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(user_id, game_id, year, iso_week)
);
CREATE INDEX idx_weekly_indiv_team ON weekly_individual_scores(team_id, game_id, year, iso_week);

CREATE TABLE weekly_team_scores (
    team_id             INT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    game_id             TEXT NOT NULL REFERENCES games(id),
    year                INT NOT NULL,
    iso_week            INT NOT NULL,
    avg_score           DOUBLE PRECISION NOT NULL,
    active_member_count BIGINT NOT NULL,
    total_member_count  BIGINT NOT NULL,
    computed_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (team_id, game_id, year, iso_week)
);

CREATE TABLE monthly_team_state (
    team_id                INT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    game_id                TEXT NOT NULL REFERENCES games(id),
    year                   INT NOT NULL,
    month                  INT NOT NULL,
    challenge_win_bonus    DOUBLE PRECISION NOT NULL DEFAULT 0,
    special_card_modifier  DOUBLE PRECISION NOT NULL DEFAULT 0,
    surprise_bonus         DOUBLE PRECISION NOT NULL DEFAULT 0,
    notes                  TEXT,
    updated_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (team_id, game_id, year, month)
);

CREATE TABLE score_adjustments (
    id              BIGSERIAL PRIMARY KEY,
    team_id         INT REFERENCES teams(id),
    user_id         INT REFERENCES users(id),
    game_id         TEXT,
    year            INT NOT NULL,
    month           INT,
    iso_week        INT,
    adjustment_type TEXT NOT NULL,  -- 'special_card', 'surprise_bonus', 'manual_correction'
    points          DOUBLE PRECISION NOT NULL,
    reason          TEXT NOT NULL,
    created_by      INT REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_adjustments_team_period ON score_adjustments(team_id, year, month);

CREATE TABLE webhook_events (
    id           BIGSERIAL PRIMARY KEY,
    source       TEXT NOT NULL,         -- 'strava'
    event_id     TEXT,
    payload      JSONB NOT NULL,
    processed    BOOLEAN NOT NULL DEFAULT false,
    error        TEXT,
    received_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at TIMESTAMPTZ,
    UNIQUE(source, event_id)
);
CREATE INDEX idx_webhook_unprocessed ON webhook_events(processed, received_at) WHERE processed = false;
