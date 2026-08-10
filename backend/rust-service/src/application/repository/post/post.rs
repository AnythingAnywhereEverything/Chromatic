use sqlx::Transaction;

use crate::application::{repository::post::row::{ HasAttachmentRow, PostLikesRow, PostRow, TotalLikedRow}, service::errors::PostServiceError};

// todo: func get YOUR FRIEND post
// todo: func get feed comment :d
// ! check visiblity
// Limit 15 on scroll FIXED //* If cause a slowness then lower it later... */
// Cursor pagnigation (created_at, Id) 
// ? snowflake is already relate to time still need created_at ?

// * / When the post has many attachment will cause the output row too many.
// * / EX: 3 Attachment and 2 tags on 1 post will cause 6 row
/*
    //*  */ Change it into json for cause match LIMIT item row output 
    //*  */ and the media_post id will not being duplicate too much 
*/
// ? How am I gonna balanced the feed between friends and normal since there's no ML for the feed
// * Schuding them for show some of there friends post
// ? Do feed setting to let user edit the feed to show friend first, no friend, normal 
//// ! BUT there's not see post in DB so the post will be always show on using Friend first how gonna 

pub async fn get_feed_public(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    cursor_id: Option<i64>,
) -> Result<Vec<PostRow>, sqlx::Error> {

    sqlx::query_as::<_, PostRow> (
        r#"
           SELECT
                m.id,
                m.user_id,
                m.content,
                m.total_comments,
                m.total_likes,
                m.reposted_from,
                m.is_repost,
                m.has_attachment,
                m.created_at,
                m.updated_at,

                COALESCE(att.attachments, '[]'::json) AS attachments,
                COALESCE(tag.tags, '[]'::json) AS tags

            FROM media_posts m

            LEFT JOIN LATERAL (
                SELECT json_agg(
                    json_build_object(
                        'id', md.id,
                        'user_id', md.user_id,
                        'media_url', md.media_url,
                        'media_preview_url', md.media_preview_url,
                        'media_category', md.media_category,
                        'media_status', md.media_status,
                        'created_at', md.created_at
                    )
                    ORDER BY md.id
                ) AS attachments
                FROM media_attachments a
                JOIN media_data md
                    ON md.id = a.target_id
                WHERE
                    a.media_id = m.id
                    AND md.media_status != 'pending'
            ) att ON TRUE

            LEFT JOIN LATERAL (
                SELECT json_agg(
                    json_build_object(
                        'id', it.id,
                        'name', it.tags_name
                    )
                    ORDER BY it.tags_name
                ) AS tags
                FROM media_tags mt
                JOIN interest_tags it
                    ON it.id = mt.interest_id
                WHERE mt.media_id = m.id
            ) tag ON TRUE

            WHERE
                m.visibility = 'everyone'
                AND m.status != 'inactive'
                AND (:cursor_id IS NULL OR m.id < :cursor_id)

            ORDER BY m.id DESC
            LIMIT 15
        "#
    )
    .bind(cursor_id)
    .fetch_all(tx.as_mut())
    .await
}


pub async fn get_friend_post(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    user_id: i64,

) {
    
}

// ! THE TAGS column has been chagne, re-new this function
pub async fn create_post(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    id:i64,
    user_id: i64,
    content: &str,
    status: &str,
    // ? attachment: Option<String>,
    media_tags: Vec<String>
) -> Result<PostRow, sqlx::Error>{
    sqlx::query_as::<_,PostRow>(
        r#"
        INSERT INTO media_posts (
        id, 
        user_id, 
        content, 
        status, 
        created_at, 
        updated_at, 
        media_tags
        )
        VALUES ($1, $2, $3, $4, NOW(), NOW(), $5)
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(content)
    .bind(status)
    .bind(media_tags)
    .fetch_one(&mut **tx)
    .await
}

// ! THE TAGS column has been chagne, re-new this function
pub async fn update_post(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    id:i64,
    user_id: i64,
    content: &str,
    status: &str,
    // attechment : Option<String>,
    media_tags: Vec<String>,
) -> Result<PostRow, sqlx::Error>{
    sqlx::query_as::<_, PostRow>(
        r#"
            UPDATE media_posts
            SET content = $1, status = $2, update_at =NOW(), media_tags = $5
            WHERE id = $3 AND user_id =$4
            RETURNING *
        "#,
    )
    .bind(content)
    .bind(status)
    .bind(id)
    .bind(user_id)
    .bind(media_tags)
    .fetch_one(&mut **tx)
    .await
}

pub async fn delete_post(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    id:i64,
    user_id: i64,
) -> Result<(), sqlx::Error>{
    sqlx::query(
        r#"
            DELETE FROM media_posts
            WHERE id = $1, user = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn add_has_attachment(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    media_id: i64,
    user_id: i64,
    target_type: String
) -> Result<Vec<HasAttachmentRow>, sqlx::Error> {
    sqlx::query_as::<_,HasAttachmentRow>(
        r#"
            INSERT INTO media_attachments( media_id,
            user_id,
            target_type)
            VALUES($1, $2,$3)
        "#
    )
    .bind(media_id)
    .bind(user_id)
    .bind(target_type)
    .fetch_all(tx.as_mut())
    .await
}

// -------------------------------------
// * Small like patch
// -------------------------------------
// ? Does this working fine????

// Pagnigation cursor
pub async fn get_info_like_person(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    post_id: i64,
    cursor_id: i64
) -> Result<Vec<PostLikesRow>, sqlx::Error> {
    sqlx::query_as(
        r#"
            SELECT *
            FROM media_likes
            WHERE medis_post_id = $1
              AND ($2 IS NULL OR id < $2)
            ORDER BY id DESC
            LIMIT 20
        "#
    )
    .bind(post_id)
    .bind(cursor_id)
    .fetch_all(tx.as_mut())
    .await
}

/*
    ?  considering between create temp table for CTE with as or 
    ?  seperate the function between INSERT : total += 1 AND DELETE : total +=1
*/ 
pub async fn like_post(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    user_id:i64,
    target_id: i64,
    is_like: bool
 ) -> Result<TotalLikedRow, sqlx::Error>{

    sqlx::query_as::<_, TotalLikedRow>(
        r#"
            WITH previous AS (
                SELECT is_like
                FROM media_likes
                WHERE user_id = $1
                  AND media_id = $2
            ),
            upsert AS (
                INSERT INTO media_likes (
                    user_id,
                    media_id,
                    created_at,
                    updated_at,
                    is_like
                )
                VALUES ($1, $2, NOW(), NOW(), $3)
                ON CONFLICT (user_id, media_id) DO UPDATE
                SET
                    is_like = EXCLUDED.is_like,
                    updated_at = NOW()
                RETURNING is_like
            )
            UPDATE media_posts
            SET total_likes = total_likes +
                CASE
                    WHEN (SELECT is_like FROM previous) IS NULL AND $3 = TRUE THEN 1
                    WHEN (SELECT is_like FROM previous) = FALSE AND $3 = TRUE THEN 1
                    WHEN (SELECT is_like FROM previous) = TRUE AND $3 = FALSE THEN -1
                    ELSE 0
                END
            WHERE id = $2
            RETURNING id, total_likes
        "#
    )
    .bind(user_id)
    .bind(target_id)
    .bind(is_like)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn bookmark_post(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    media_id: i64,
    user_id: i64
) -> Result<(), PostServiceError> {
    let _ = sqlx::query(
        r#"
            INSERT INTO media_bookmarks (post_id, user_id, created_at)
            VALUES ($1, $2 , NOW())
        "#
    )
    .bind(media_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await;

    Ok(())
}

// * Not so Standard
pub async fn remove_bookmark_post(
    tx: &mut Transaction<'_,sqlx::Postgres>,
    media_id: i64,
    user_id: i64
) -> Result<(), PostServiceError> {
    let _ = sqlx::query(
        r#"
            DELETE FROM media_bookmarks
            WHERE media_id = $1 AND user_id = $2
        "#
    )
    .bind(media_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await;
    Ok(())
}