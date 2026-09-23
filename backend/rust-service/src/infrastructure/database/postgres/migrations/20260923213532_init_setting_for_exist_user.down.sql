-- Add down migration script here
DELETE FROM user_settings
WHERE setting_key IN (
    'notification',
    'privacy',
    'message'
);