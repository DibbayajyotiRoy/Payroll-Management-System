-- Add new columns to the employees table to support advanced features.
ALTER TABLE employees
ADD COLUMN employee_id TEXT UNIQUE NOT NULL DEFAULT ('EMP-' || LPAD(CAST(FLOOR(RANDOM() * 100000) AS TEXT), 6, '0')),
ADD COLUMN phone TEXT,
ADD COLUMN department TEXT,
ADD COLUMN position TEXT,
ADD COLUMN employment_type JSONB, -- To store enum like 'FullTime', 'PartTime'
ADD COLUMN status JSONB, -- To store enum like 'Active', 'OnLeave'
ADD COLUMN salary_info JSONB, -- To store SalaryInfo struct
ADD COLUMN benefits JSONB, -- To store array of Benefit structs
ADD COLUMN tax_info JSONB, -- To store TaxInfo struct
ADD COLUMN banking_info JSONB, -- To store BankingInfo struct (should be encrypted)
ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Optional: Create a trigger to automatically update the updated_at timestamp.
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON employees
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();