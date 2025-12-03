// Clipboard history storage using SQLite
use crate::app::error::AppResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardHistoryItem {
    pub id: String,
    pub content_type: String,
    pub content_hash: String,
    pub plain_text: Option<String>,
    pub data: Option<Vec<u8>>,
    pub source_app: Option<String>,
    pub source_window: Option<String>,
    pub is_favorite: bool,
    pub is_sensitive: bool,
    pub created_at: DateTime<Utc>,
    pub accessed_at: Option<DateTime<Utc>>,
    pub access_count: i32,
}

pub struct ClipboardStorage {
    pool: SqlitePool,
}

impl ClipboardStorage {
    pub async fn new(pool: SqlitePool) -> AppResult<Self> {
        let storage = Self { pool };
        storage.initialize_schema().await?;
        Ok(storage)
    }

    async fn initialize_schema(&self) -> AppResult<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS clipboard_history (
                id TEXT PRIMARY KEY,
                content_type TEXT NOT NULL,
                content_hash TEXT NOT NULL,
                plain_text TEXT,
                data BLOB,
                source_app TEXT,
                source_window TEXT,
                is_favorite BOOLEAN DEFAULT FALSE,
                is_sensitive BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                accessed_at TIMESTAMP,
                access_count INTEGER DEFAULT 0
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for better query performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_created_at ON clipboard_history(created_at DESC)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_content_hash ON clipboard_history(content_hash)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_is_favorite ON clipboard_history(is_favorite) WHERE is_favorite = TRUE")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Add a new clipboard item to history
    pub async fn add_item(&self, item: &ClipboardHistoryItem) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO clipboard_history (
                id, content_type, content_hash, plain_text, data,
                source_app, source_window, is_favorite, is_sensitive,
                created_at, accessed_at, access_count
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&item.id)
        .bind(&item.content_type)
        .bind(&item.content_hash)
        .bind(&item.plain_text)
        .bind(&item.data)
        .bind(&item.source_app)
        .bind(&item.source_window)
        .bind(item.is_favorite)
        .bind(item.is_sensitive)
        .bind(item.created_at)
        .bind(item.accessed_at)
        .bind(item.access_count)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get clipboard history with pagination
    pub async fn get_history(&self, limit: i32, offset: i32) -> AppResult<Vec<ClipboardHistoryItem>> {
        let rows = sqlx::query(
            r#"
            SELECT id, content_type, content_hash, plain_text, data,
                   source_app, source_window, is_favorite, is_sensitive,
                   created_at, accessed_at, access_count
            FROM clipboard_history
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let items = rows
            .into_iter()
            .map(|row| ClipboardHistoryItem {
                id: row.get("id"),
                content_type: row.get("content_type"),
                content_hash: row.get("content_hash"),
                plain_text: row.get("plain_text"),
                data: row.get("data"),
                source_app: row.get("source_app"),
                source_window: row.get("source_window"),
                is_favorite: row.get("is_favorite"),
                is_sensitive: row.get("is_sensitive"),
                created_at: row.get("created_at"),
                accessed_at: row.get("accessed_at"),
                access_count: row.get("access_count"),
            })
            .collect();

        Ok(items)
    }

    /// Search clipboard history by text
    pub async fn search(&self, query: &str) -> AppResult<Vec<ClipboardHistoryItem>> {
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query(
            r#"
            SELECT id, content_type, content_hash, plain_text, data,
                   source_app, source_window, is_favorite, is_sensitive,
                   created_at, accessed_at, access_count
            FROM clipboard_history
            WHERE plain_text LIKE ?
            ORDER BY created_at DESC
            LIMIT 50
            "#,
        )
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        let items = rows
            .into_iter()
            .map(|row| ClipboardHistoryItem {
                id: row.get("id"),
                content_type: row.get("content_type"),
                content_hash: row.get("content_hash"),
                plain_text: row.get("plain_text"),
                data: row.get("data"),
                source_app: row.get("source_app"),
                source_window: row.get("source_window"),
                is_favorite: row.get("is_favorite"),
                is_sensitive: row.get("is_sensitive"),
                created_at: row.get("created_at"),
                accessed_at: row.get("accessed_at"),
                access_count: row.get("access_count"),
            })
            .collect();

        Ok(items)
    }

    /// Toggle favorite status
    pub async fn toggle_favorite(&self, id: &str) -> AppResult<bool> {
        let current: bool = sqlx::query_scalar("SELECT is_favorite FROM clipboard_history WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let new_value = !current;
        sqlx::query("UPDATE clipboard_history SET is_favorite = ? WHERE id = ?")
            .bind(new_value)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(new_value)
    }

    /// Delete a clipboard item
    pub async fn delete_item(&self, id: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM clipboard_history WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Clear all clipboard history
    pub async fn clear_all(&self) -> AppResult<()> {
        sqlx::query("DELETE FROM clipboard_history WHERE is_favorite = FALSE")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Record that an item was accessed
    pub async fn record_access(&self, id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE clipboard_history
            SET accessed_at = ?, access_count = access_count + 1
            WHERE id = ?
            "#,
        )
        .bind(Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Check if content already exists in history
    pub async fn exists_by_hash(&self, content_hash: &str) -> AppResult<bool> {
        let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM clipboard_history WHERE content_hash = ?")
            .bind(content_hash)
            .fetch_one(&self.pool)
            .await?;

        Ok(count > 0)
    }

    /// Get favorites only
    pub async fn get_favorites(&self) -> AppResult<Vec<ClipboardHistoryItem>> {
        let rows = sqlx::query(
            r#"
            SELECT id, content_type, content_hash, plain_text, data,
                   source_app, source_window, is_favorite, is_sensitive,
                   created_at, accessed_at, access_count
            FROM clipboard_history
            WHERE is_favorite = TRUE
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let items = rows
            .into_iter()
            .map(|row| ClipboardHistoryItem {
                id: row.get("id"),
                content_type: row.get("content_type"),
                content_hash: row.get("content_hash"),
                plain_text: row.get("plain_text"),
                data: row.get("data"),
                source_app: row.get("source_app"),
                source_window: row.get("source_window"),
                is_favorite: row.get("is_favorite"),
                is_sensitive: row.get("is_sensitive"),
                created_at: row.get("created_at"),
                accessed_at: row.get("accessed_at"),
                access_count: row.get("access_count"),
            })
            .collect();

        Ok(items)
    }

    /// Get a single item by ID
    pub async fn get_by_id(&self, id: &str) -> AppResult<Option<ClipboardHistoryItem>> {
        let row = sqlx::query(
            r#"
            SELECT id, content_type, content_hash, plain_text, data,
                   source_app, source_window, is_favorite, is_sensitive,
                   created_at, accessed_at, access_count
            FROM clipboard_history
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| ClipboardHistoryItem {
            id: row.get("id"),
            content_type: row.get("content_type"),
            content_hash: row.get("content_hash"),
            plain_text: row.get("plain_text"),
            data: row.get("data"),
            source_app: row.get("source_app"),
            source_window: row.get("source_window"),
            is_favorite: row.get("is_favorite"),
            is_sensitive: row.get("is_sensitive"),
            created_at: row.get("created_at"),
            accessed_at: row.get("accessed_at"),
            access_count: row.get("access_count"),
        }))
    }

    /// Delete a clipboard item (alias for delete_item)
    pub async fn delete(&self, id: &str) -> AppResult<()> {
        self.delete_item(id).await
    }

    /// Increment access count for an item
    pub async fn increment_access_count(&self, id: &str) -> AppResult<()> {
        self.record_access(id).await
    }
}
