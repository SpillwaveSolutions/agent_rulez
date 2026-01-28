# Database Schema Migration - Manual Execution Guide

**Date:** 2025-12-31
**Estimated Time:** 15 minutes
**Difficulty:** Medium
**Corresponding Code Agent Instructions:** `instructions/instruct-20251231_143000-db-migration.md`

---

## Prerequisites

Before starting, ensure:

- [ ] PostgreSQL 14+ is installed and running
- [ ] You have admin access to the database
- [ ] A backup of the current database exists
- [ ] No active transactions are running against the database

### Environment Setup

```bash
# Set database connection (adjust for your environment)
export DATABASE_URL="postgresql://admin:secret@localhost:5432/myapp_dev"

# Verify you can connect
psql $DATABASE_URL -c "SELECT version();"
```

Expected output:
```
                                   version
---------------------------------------------------------------------------
 PostgreSQL 14.5 on x86_64-apple-darwin21.6.0, compiled by Apple clang...
```

---

## Overview

### What This Accomplishes
This migration adds a `last_login_at` timestamp column to the `users` table and creates an index for efficient querying of recent logins. This supports the new "active users" dashboard feature.

### What You'll Do
1. Create a backup of the users table
2. Add the new column with a default value
3. Create an index for query performance
4. Verify the migration succeeded

---

## Step 1: Create Backup

### Why This Step
Before modifying any schema, we create a backup so we can restore if something goes wrong. This is especially important for the users table since it contains critical authentication data.

### Commands

```bash
# Create a timestamped backup of the users table
BACKUP_FILE="users_backup_$(date +%Y%m%d_%H%M%S).sql"

pg_dump $DATABASE_URL \
  --table=users \
  --data-only \
  --file="$BACKUP_FILE"

echo "Backup created: $BACKUP_FILE"
```

### Expected Output

```
Backup created: users_backup_20251231_143022.sql
```

### Verification

```bash
# Confirm backup file exists and has content
ls -la users_backup_*.sql
wc -l users_backup_*.sql
```

Expected:
```
-rw-r--r--  1 user  staff  45231 Dec 31 14:30 users_backup_20251231_143022.sql
    1247 users_backup_20251231_143022.sql
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| `connection refused` | PostgreSQL not running | `brew services start postgresql` (macOS) or `sudo systemctl start postgresql` (Linux) |
| `permission denied` | Wrong user permissions | Connect as superuser: `psql -U postgres` |
| `pg_dump: command not found` | PostgreSQL tools not in PATH | Add to PATH: `export PATH="/usr/local/pgsql/bin:$PATH"` |

---

## Step 2: Add Column

### Why This Step
We add the `last_login_at` column with a NULL default so existing rows don't need immediate values. Using `IF NOT EXISTS` makes this idempotent - safe to run multiple times.

### Commands

```bash
psql $DATABASE_URL << 'EOF'
-- Add the new column (idempotent)
ALTER TABLE users
ADD COLUMN IF NOT EXISTS last_login_at TIMESTAMP WITH TIME ZONE;

-- Verify column was added
SELECT column_name, data_type, is_nullable
FROM information_schema.columns
WHERE table_name = 'users' AND column_name = 'last_login_at';
EOF
```

### Expected Output

```
ALTER TABLE
 column_name   |        data_type         | is_nullable
---------------+--------------------------+-------------
 last_login_at | timestamp with time zone | YES
(1 row)
```

### Verification

```bash
# Double-check the column exists with correct type
psql $DATABASE_URL -c "\d users" | grep last_login_at
```

Expected:
```
 last_login_at     | timestamp with time zone |           |          |
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| `column "last_login_at" already exists` | Column exists (not using IF NOT EXISTS) | Safe to proceed - column already there |
| `permission denied for table users` | Insufficient privileges | Connect as table owner or superuser |
| `syntax error` | PostgreSQL version incompatibility | Check `SELECT version();` - IF NOT EXISTS requires PostgreSQL 9.6+ |

---

## Step 3: Create Index

### Why This Step
We create an index on `last_login_at` because the active users dashboard will query `WHERE last_login_at > NOW() - INTERVAL '30 days'`. Without an index, this query would do a full table scan.

### Commands

```bash
psql $DATABASE_URL << 'EOF'
-- Create index for efficient date range queries
-- CONCURRENTLY prevents locking the table during creation
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_last_login_at
ON users (last_login_at DESC NULLS LAST);

-- Verify index was created
SELECT indexname, indexdef
FROM pg_indexes
WHERE tablename = 'users' AND indexname = 'idx_users_last_login_at';
EOF
```

### Expected Output

```
CREATE INDEX
       indexname           |                           indexdef
---------------------------+--------------------------------------------------------------
 idx_users_last_login_at   | CREATE INDEX idx_users_last_login_at ON public.users USING btree (last_login_at DESC NULLS LAST)
(1 row)
```

### Verification

```bash
# Confirm index is valid (not in-progress or invalid)
psql $DATABASE_URL -c "SELECT indexrelid::regclass, indisvalid FROM pg_index WHERE indexrelid = 'idx_users_last_login_at'::regclass;"
```

Expected:
```
        indexrelid         | indisvalid
---------------------------+------------
 idx_users_last_login_at   | t
(1 row)
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| `CONCURRENTLY cannot be used in transaction` | Inside a transaction block | Run outside transaction: `psql -c "CREATE INDEX..."` not `psql << EOF` |
| `index "idx_users_last_login_at" already exists` | Index exists | Safe to proceed if `indisvalid = t` |
| `canceling statement due to lock timeout` | Table is locked | Retry during low-traffic period |

---

## Final Verification

After completing all steps, verify the entire migration succeeded:

```bash
psql $DATABASE_URL << 'EOF'
-- Comprehensive verification
SELECT
  (SELECT COUNT(*) FROM information_schema.columns
   WHERE table_name = 'users' AND column_name = 'last_login_at') as column_exists,
  (SELECT COUNT(*) FROM pg_indexes
   WHERE tablename = 'users' AND indexname = 'idx_users_last_login_at') as index_exists;
EOF
```

### Expected Final State

| Metric | Expected Value |
|--------|---------------|
| column_exists | 1 |
| index_exists | 1 |

If both values are 1, the migration was successful.

---

## Rollback Procedure

If something goes wrong and you need to undo the migration:

```bash
# Remove the index
psql $DATABASE_URL -c "DROP INDEX IF EXISTS idx_users_last_login_at;"

# Remove the column
psql $DATABASE_URL -c "ALTER TABLE users DROP COLUMN IF EXISTS last_login_at;"

# If data was corrupted, restore from backup
psql $DATABASE_URL < users_backup_20251231_143022.sql
```

---

## Summary

| Step | What You Did | Time |
|------|--------------|------|
| 1 | Created backup of users table | 1 min |
| 2 | Added last_login_at column | 1 min |
| 3 | Created descending index | 2-10 min (depends on table size) |
| 4 | Verified migration | 1 min |
| **Total** | | **5-13 min** |

---

## Next Steps

After completing this task:
- [ ] Notify backend team that migration is complete
- [ ] Update API endpoint to populate `last_login_at` on login
- [ ] Deploy the active users dashboard feature
- [ ] Delete backup file after 7 days if no issues: `rm users_backup_*.sql`
