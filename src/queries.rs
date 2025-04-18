use crate::models::{Channel, Message, User};
use crate::AppState;
use sqlx::Result;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_channels(data: Arc<AppState>) -> Result<Vec<Channel>> {
    return sqlx::query_as!(
        Channel,
        r#"
        SELECT
            *
        FROM channels
        WHERE deleted_at IS NULL
        ORDER BY sort_order ASC
        "#
    )
    .fetch_all(&data.db)
    .await;
}

pub async fn get_channel_messages(data: Arc<AppState>, channel_id: String) -> Result<Vec<Message>> {
    return sqlx::query_as!(
        Message,
        r#"
        SELECT
            x.*
        FROM (
            SELECT
                *
            FROM messages
            WHERE deleted_at IS NULL
            AND channel_id = ?
            ORDER BY created_at DESC
            LIMIT 100
        ) x
        ORDER BY x.created_at ASC
        "#,
        channel_id
    )
    .fetch_all(&data.db)
    .await;
}

pub async fn get_user_by_id(data: Arc<AppState>, user_id: String) -> Result<User> {
    sqlx::query_as!(
        User,
        r#"
        SELECT
            *
        FROM users
        WHERE id = ?
        "#,
        user_id
    )
    .fetch_one(&data.db)
    .await
}

pub async fn get_users(data: Arc<AppState>, user_ids: Option<&[String]>) -> Result<Vec<User>> {
    if let Some(ids) = user_ids {
        if ids.is_empty() {
            // Early return: no users requested
            return Ok(vec![]);
        }

        // Dynamically generate placeholders for `IN (?, ?, ?)`
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let sql = format!("SELECT * FROM users WHERE id IN ({})", placeholders);

        let mut query = sqlx::query_as::<_, User>(&sql);

        for id in ids {
            query = query.bind(id);
        }

        query.fetch_all(&data.db).await
    } else {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            "#
        )
        .fetch_all(&data.db)
        .await
    }
}

// pub async fn get_user_by_id(data: Arc<AppState>, user_id: String) -> Result<User> {
//     return sqlx::query_as!(
//         User,
//         r#"
//         SELECT
//             *
//         FROM users
//         WHERE id = ?
//         "#,
//         user_id
//     )
//         .fetch_one(&data.db)
//         .await
// }

pub async fn create_message(
    data: Arc<AppState>,
    user_id: String,
    channel_id: String,
    content: Option<String>,
) -> Result<Message> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO messages (id, user_id, channel_id, content, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        id,
        user_id,
        channel_id,
        content,
        now,
        now,
    )
    .execute(&data.db)
    .await?;

    let message = sqlx::query_as!(
        Message,
        r#"
        SELECT
            *
        FROM messages
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(&data.db)
    .await?;

    Ok(message)
}
