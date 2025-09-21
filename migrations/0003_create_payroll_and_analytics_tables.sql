-- Table for tracking individual payroll runs
CREATE TABLE payroll_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    run_type JSONB NOT NULL, -- 'Regular', 'Bonus', 'EarnedWageAccess'
    status JSONB NOT NULL, -- 'Draft', 'Processing', 'Completed'
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    total_employees INT,
    total_amount NUMERIC(15, 2), -- Suitable for currency
    ai_validation_score FLOAT, -- AI confidence in payroll accuracy
    blockchain_hash TEXT, -- Immutable record hash from a blockchain
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);

-- Table for storing AI-powered analytics and insights
CREATE TABLE payroll_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    total_gross_pay NUMERIC(15, 2),
    total_deductions NUMERIC(15, 2),
    total_net_pay NUMERIC(15, 2),
    employee_count INT,
    ai_insights JSONB, -- To store AIInsights struct (cost predictions, anomalies)
    fraud_score FLOAT NOT NULL, -- AI-calculated fraud risk score
    compliance_score FLOAT NOT NULL, -- AI-calculated compliance score
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_payroll_run FOREIGN KEY(id) REFERENCES payroll_runs(id) ON DELETE CASCADE -- Optional link
);

-- Add indexes for performance on frequently queried columns
CREATE INDEX idx_payroll_runs_period ON payroll_runs (period_start, period_end);
CREATE INDEX idx_analytics_period ON payroll_analytics (period_start, period_end);