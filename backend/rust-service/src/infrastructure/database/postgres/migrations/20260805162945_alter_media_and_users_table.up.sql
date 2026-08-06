-- Add up migration script here

ALTER TABLE media_likes 
    ADD is_liked bool NULL;

ALTER TABLE media_posts
    ADD media_tags TEXT[];

ALTER TABLE users
    ADD interest_tags TEXT[];

ALTER TABLE media_posts
    ADD visibility varchar(20);

-- INDEX for perfromance. I hope so
CREATE INDEX media_posts_tags_x ON media_posts USING GIN (media_tags);
CREATE INDEX users_interest_tags_x ON users USING GIN (interest_tags);