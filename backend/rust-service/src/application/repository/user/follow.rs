use crate::application::repository::user::row::FollowUserRow;
pub async fn is_following(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Option<i64>,
    follower_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar(
        r#"
    SELECT EXISTS(
        SELECT 1
        FROM user_follow
        WHERE user_id = $1 AND follower_id = $2
    )
    "#,
    )
    .bind(user_id)
    .bind(follower_id)
    .fetch_one(tx.as_mut())
    .await?;
    Ok(result)
}

pub async fn follow_repo(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    follow_id: i64,
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
    .bind(user_id)
    .bind(follow_id)
    .bind(status)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn unfollow_repo(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    follower_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
    DELETE FROM user_follow
    WHERE user_id = $1 AND follower_id = $2
    "#,
    )
    .bind(user_id)
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