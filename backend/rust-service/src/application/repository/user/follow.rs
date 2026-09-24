use crate::application::repository::user::row::{FollowUserRow, PendingFollowRow};

// Row orientation follows the schema: user_id = the user being followed,
// follower_id = the user who requested/owns the follow.
pub async fn is_following(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    follower_id: Option<i64>,
    followed_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar(
        r#"
    SELECT EXISTS(
        SELECT 1
        FROM user_follow
        WHERE user_id = $2 AND follower_id = $1
    )
    "#,
    )
    .bind(follower_id)
    .bind(followed_id)
    .fetch_one(tx.as_mut())
    .await?;
    Ok(result)
}

pub async fn follow_repo(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    followed_id: i64,
    follower_id: i64,
    status: &str,
) -> Result<FollowUserRow, sqlx::Error> {
    sqlx::query_as::<_, FollowUserRow>(
        r#"
    INSERT INTO user_follow (
        user_id,
        follower_id,
        status,
        created_at
    )
    VALUES ($1, $2, $3, NOW())
    ON CONFLICT (user_id, follower_id)
    DO UPDATE SET
        status = EXCLUDED.status
    RETURNING user_id::text  as user_id, follower_id::text as follower_id, status, created_at
        
    "#,
    )
    .bind(followed_id)
    .bind(follower_id)
    .bind(status)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn list_pending_followers(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    followed_id: i64,
) -> Result<Vec<PendingFollowRow>, sqlx::Error> {
    sqlx::query_as::<_, PendingFollowRow>(
        r#"
    SELECT
        uf.follower_id::TEXT AS follower_id,
        u.username,
        up.display_name,
        m1.name AS avatar,
        m1.thumbhash AS avatar_thumbhash,
        uf.status,
        uf.created_at
    FROM user_follow uf
    JOIN users u
        ON u.id = uf.follower_id
    LEFT JOIN user_profiles up
        ON up.user_id = u.id
    LEFT JOIN media_objects m1
        ON up.avatar_media_id = m1.media_id AND m1.kind = 'original'
    WHERE uf.user_id = $1
      AND uf.status = 'pending'
      AND u.deleted_at IS NULL
    ORDER BY uf.created_at DESC
    "#,
    )
    .bind(followed_id)
    .fetch_all(tx.as_mut())
    .await
}

pub async fn reject_follow_repo(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    followed_id: i64,
    follower_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
    DELETE FROM user_follow
    WHERE user_id = $1 AND follower_id = $2 AND status = 'pending'
    "#,
    )
    .bind(followed_id)
    .bind(follower_id)
    .execute(tx.as_mut())
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn unfollow_repo(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    followed_id: i64,
    follower_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
    DELETE FROM user_follow
    WHERE user_id = $1 AND follower_id = $2
    "#,
    )
    .bind(followed_id)
    .bind(follower_id)
    .execute(tx.as_mut())
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn update_follower_count(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    delta: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
    UPDATE user_profiles
    SET followers_count = followers_count + $2
    WHERE user_id = $1;
    "#,
    )
    .bind(user_id)
    .bind(delta)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn update_following_count(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    delta: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
    UPDATE user_profiles
    SET following_count = following_count + $2
    WHERE user_id = $1;
    "#,
    )
    .bind(user_id)
    .bind(delta)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}