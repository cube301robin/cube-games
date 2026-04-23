-- Add fixed roster size to teams table.
-- This is the canonical denominator for team averages — it does not change
-- as athletes onboard their Strava accounts.
ALTER TABLE teams ADD COLUMN roster_size INT NOT NULL DEFAULT 0;
