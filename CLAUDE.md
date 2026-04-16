# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build entire workspace
cargo build

# Run the API server (auto-runs sqlx migrations on startup)
cargo run -p api

# Run the scheduler (separate process alongside the API)
cargo run -p scheduler

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p scoring

# Run a single test by name
cargo test -p scoring tier_boundaries

# Check without building
cargo check

# Start local Postgres + (optional) Redis
docker-compose up -d
```

## Architecture

This is a Cargo workspace with 6 crates. Dependency flow: `core` ← `db` ← `scoring`, `strava`, `api`, `scheduler`.

**`crates/core`** — `AppConfig` loaded via `config-rs` (reads `config/base.toml`, `config/local.toml`, then `CUBEGAMES__*` env vars). `AppError` shared error enum.

**`crates/db`** — All sqlx queries. `models.rs` has DB row structs and response DTOs. `queries.rs` has the named query functions used by handlers. Uses `DOUBLE PRECISION` (not `REAL`) in Postgres to match Rust `f64`.

**`crates/scoring`** — `GameScoring` trait with `compute_weekly_individual` and `compute_weekly_team`. `ScoringRegistry` is a `HashMap<String, Arc<dyn GameScoring>>` keyed by game ID. Adding a new game: implement the trait in a new file, register in `ScoringRegistry::new()`, and `INSERT INTO games`. No other files change.

**`crates/strava`** — Strava uses a non-standard OAuth2 flow (custom token exchange endpoint returns the athlete inline). `StravaOAuth::exchange_code` hits `strava.com/oauth/token` directly rather than using the oauth2 crate's standard flow.

**`crates/api`** — Axum app state is `AppState` (db pool, scoring registry, strava client/oauth, config). JWT auth via `AuthUser` and `AdminUser` extractors in `auth.rs`. Static `frontend/` directory is served as a fallback via `ServeDir`. Routes are split by domain under `handlers/`.

**`crates/scheduler`** — Three jobs: weekly score rollup (Sun 16:59 UTC = midnight Bangkok), hourly Strava token refresh, 30s webhook queue processor. The webhook processor reads unprocessed rows from `webhook_events`, fetches the full activity from Strava, and writes to `activities`.

## Key Design Constraints

- **Timezone**: Scores are rolled up at Bangkok midnight (Asia/Bangkok = UTC+7). ISO weeks computed in Bangkok TZ. Data stored as `TIMESTAMPTZ`.
- **Idempotency**: All score writes use `ON CONFLICT ... DO UPDATE`. Re-running a rollup is always safe.
- **Team average uses full roster**: `weekly_team_scores.total_member_count` includes inactive members — this is intentional per the PRD.
- **Activity deduplication**: `activities` has `UNIQUE(source, external_id)`, so replayed Strava webhooks are silently ignored.
- **Scoring is read from DB, not config**: The `walk_run_2026` game row is seeded in the migration. Tier thresholds live in `WalkRunGame::tier_for_km` in `crates/scoring/src/walk_run.rs`.

## Configuration

`config/base.toml` is the baseline. `config/local.toml` (gitignored) overrides locally. Env vars override both using the prefix `CUBEGAMES__` with `__` as separator (e.g. `CUBEGAMES__STRAVA__CLIENT_ID`). `DATABASE_URL` is also read directly for sqlx CLI compatibility.

## Frontend

Four static HTML pages in `frontend/` using CDN Tailwind with a custom dark theme. Pages are currently populated with placeholder data — they need JS `fetch()` calls wired to `/api/scoreboard/individuals` and `/api/scoreboard/teams`. The Tailwind config (colors, fonts) is identical across all four pages and must stay in sync if the design system changes.
