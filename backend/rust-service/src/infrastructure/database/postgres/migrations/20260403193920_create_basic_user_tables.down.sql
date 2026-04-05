-- Add down migration script here

-- Drop user, credentials, and profile tables
DROP TABLE IF EXISTS user_profiles;
DROP TABLE IF EXISTS user_credentials;
DROP TABLE IF EXISTS users;