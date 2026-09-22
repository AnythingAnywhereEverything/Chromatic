-- Add up migration script here
ALTER TABLE interest_tags
DROP COLUMN IF EXISTS tag_color;

-- * University major / field-of-study tags

INSERT INTO interest_tags (id, tag_name)
SELECT v.id, v.tag_name
FROM (
    VALUES
        (1, 'Computer Science'),
        (2, 'Information Technology'),
        (3, 'Software Engineering'),
        (4, 'Data Science'),
        (5, 'Artificial Intelligence'),
        (6, 'Electrical Engineering'),
        (7, 'Mechanical Engineering'),
        (8, 'Civil Engineering'),
        (9, 'Mathematics'),
        (10, 'Physics'),
        (11, 'Chemistry'),
        (12, 'Biology'),
        (13, 'Medicine'),
        (14, 'Business Administration'),
        (15, 'Economics'),
        (16, 'Accounting'),
        (17, 'Psychology'),
        (18, 'Education'),
        (19, 'Law'),
        (20, 'Architecture'),
        (21, 'Political Science'),
        (22, 'Sociology'),
        (23, 'History'),
        (24, 'Languages'),
        (25, 'Literature'),
        (26, 'Fine Arts'),
        (27, 'Music')
) AS v(id, tag_name)
WHERE NOT EXISTS (
    SELECT 1
    FROM interest_tags t
    WHERE t.tag_name = v.tag_name
);