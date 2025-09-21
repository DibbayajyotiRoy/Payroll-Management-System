CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE employees (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  hire_date DATE NOT NULL DEFAULT CURRENT_DATE
);
