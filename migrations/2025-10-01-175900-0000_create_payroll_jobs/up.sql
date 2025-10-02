-- Conditionally create the ENUM type to prevent errors on re-runs.
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'payroll_run_status') THEN
        CREATE TYPE payroll_run_status AS ENUM ('pending', 'running', 'completed', 'failed');
    END IF;
END$$;

-- Stores the user's configuration for when their payroll should run.
CREATE TABLE IF NOT EXISTS payroll_schedules (
    id SERIAL PRIMARY KEY,
    company_id VARCHAR NOT NULL UNIQUE REFERENCES company_configurations(company_id) ON DELETE CASCADE,
    run_day_of_month SMALLINT NOT NULL CHECK (run_day_of_month >= 1 AND run_day_of_month <= 31),
    is_active BOOLEAN NOT NULL DEFAULT FALSE, -- The on/off switch for the user
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Tracks the history and status of each individual automated payroll run.
CREATE TABLE IF NOT EXISTS payroll_runs (
    id UUID PRIMARY KEY,
    company_id VARCHAR NOT NULL REFERENCES company_configurations(company_id),
    schedule_id INTEGER NOT NULL REFERENCES payroll_schedules(id),
    pay_period_start DATE NOT NULL,
    pay_period_end DATE NOT NULL,
    status payroll_run_status NOT NULL DEFAULT 'pending',
    results JSONB, -- Stores the final PayrollResponse.results
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);

-- Helper function to automatically update `updated_at` timestamps.
-- This function is idempotent by nature, so no change needed.
SELECT diesel_manage_updated_at('payroll_schedules');