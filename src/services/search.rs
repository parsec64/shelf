use std::sync::Arc;

use sqlx::{SqlitePool, prelude::FromRow};


#[derive(Debug, Clone, FromRow)]
pub struct SearchRow {
    pub id: i32,
    pub name: Option<String>,
    pub content: String,
    pub item_type: String,
    pub group_id: i32,
    pub group_name: String,
}

pub async fn search_items(
    pool: Arc<SqlitePool>,
    query: String,
    limit: i32,
) -> Result<Vec<SearchRow>, String> {
    let query = query.trim();

    if query.is_empty() {
        return Ok(vec![]);
    }

    // Escape LIKE wildcards so '%' and '_' typed by the user match literally
    let escaped = query
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    let pattern = format!("%{}%", escaped);

    sqlx::query_as::<_, SearchRow>(
        r#"
        SELECT DISTINCT
            i.id,
            i.name,
            i.content,
            i.item_type,
            g.id   AS group_id,
            g.name AS group_name
        FROM items i
        INNER JOIN groups g ON g.id = i.group_id
        LEFT JOIN tags t ON t.item_id = i.id
        WHERE
            i.content LIKE ? COLLATE NOCASE ESCAPE '\'
            OR IFNULL(i.name, '') LIKE ? COLLATE NOCASE ESCAPE '\'
            OR IFNULL(t.name, '') LIKE ? COLLATE NOCASE ESCAPE '\'
        ORDER BY
            CASE
                WHEN IFNULL(i.name, '') LIKE ? COLLATE NOCASE ESCAPE '\' THEN 0
                WHEN i.content LIKE ? COLLATE NOCASE ESCAPE '\' THEN 1
                ELSE 2
            END,
            g.name,
            i.id DESC
        LIMIT ?
        "#,
    )
    .bind(&pattern)
    .bind(&pattern)
    .bind(&pattern)
    .bind(&pattern) // name rank
    .bind(&pattern) // content rank
    .bind(limit)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())
}
