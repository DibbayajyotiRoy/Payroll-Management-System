-- Use CASCADE to automatically drop dependent objects (like foreign keys)
DROP TABLE IF EXISTS attendance_records CASCADE;
DROP TABLE IF EXISTS leave_policies CASCADE;
DROP TABLE IF EXISTS role_configurations CASCADE;
DROP TABLE IF EXISTS leave_types CASCADE;
DROP TABLE IF EXISTS company_configurations CASCADE;
DROP TYPE IF EXISTS attendance_status_enum CASCADE;