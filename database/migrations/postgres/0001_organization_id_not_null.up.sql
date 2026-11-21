-- sdkwork:migration
-- id: 0001_organization_id_not_null
-- engine: postgres
-- module: sdkwork-mahjong
-- purpose: Enforce organization_id NOT NULL DEFAULT on all tables in the
--   consolidated baseline. NULL rows (pre-standard data anomalies) are
--   backfilled with the platform sentinel before NOT NULL is set, and
--   NOT NULL columns without an explicit default receive the sentinel
--   default, keeping existing deployments consistent with fresh baseline
--   installs.
-- reversible: false
-- rollback: forward-fix (sentinel backfill is the canonical fix; NULL
--   organization rows are data anomalies)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

ALTER TABLE mj_match ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '0';
UPDATE mj_match SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE mj_match ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE mj_match ALTER COLUMN organization_id SET NOT NULL;

ALTER TABLE mj_room ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '0';
UPDATE mj_room SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE mj_room ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE mj_room ALTER COLUMN organization_id SET NOT NULL;

ALTER TABLE mj_leaderboard ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '0';
UPDATE mj_leaderboard SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE mj_leaderboard ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE mj_leaderboard ALTER COLUMN organization_id SET NOT NULL;

COMMIT;
