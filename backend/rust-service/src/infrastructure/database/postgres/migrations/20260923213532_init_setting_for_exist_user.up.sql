-- Add up migration script here
INSERT INTO user_settings (
    user_id,
    setting_key,
    setting_value,
    created_at,
    updated_at
)
SELECT
    u.id,
    s.setting_key,
    s.setting_value,
    NOW(),
    NOW()
FROM users u
CROSS JOIN (
    VALUES
        (
            'notification',
            '{
                "post": {
                    "following_person": 1,
                    "likes": "follower",
                    "tags": "everyone",
                    "comments": "follower",
                    "comment_likes": 1,
					"new_post" : "follower"
                },
                "follow": {
                    "follow_request": 1
                },
                "message": {
                    "new_message": 1,
                    "message_requests": 1
                }
            }'::jsonb
        ),
        (
            'privacy',
            '{
                "profile_visibility": "everyone",
                "who_can_follow_me": "everyone"
            }'::jsonb
        ),
        (
            'message',
            '{
                "message_requests": "everyone",
                "add_to_message_group": "everyone"
            }'::jsonb
        )
) AS s(setting_key, setting_value)
ON CONFLICT (user_id, setting_key) DO NOTHING;