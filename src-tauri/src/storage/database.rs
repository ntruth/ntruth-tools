// Database module for SQLite operations
use crate::app::error::{AppError, AppResult};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

/// Database connection manager
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection
    pub async fn new(db_path: &Path) -> AppResult<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Create connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&format!("sqlite://{}", db_path.display()))
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let db = Self { pool };

        // Run migrations
        db.run_migrations().await?;

        Ok(db)
    }

    /// Run database migrations to create tables
    async fn run_migrations(&self) -> AppResult<()> {
        // File index table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS file_index (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                extension TEXT,
                size INTEGER,
                modified_at INTEGER,
                indexed_at INTEGER,
                access_count INTEGER DEFAULT 0,
                last_accessed_at INTEGER
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Clipboard history table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS clipboard_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_type TEXT NOT NULL,
                content TEXT NOT NULL,
                preview TEXT,
                source_app TEXT,
                is_favorite INTEGER DEFAULT 0,
                created_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // App usage statistics table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS app_usage (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                app_path TEXT NOT NULL UNIQUE,
                app_name TEXT NOT NULL,
                launch_count INTEGER DEFAULT 0,
                last_launched_at INTEGER
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Search history table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS search_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                query TEXT NOT NULL,
                result_type TEXT,
                result_id TEXT,
                searched_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // AI conversations table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS ai_conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                provider TEXT NOT NULL,
                model TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // AI messages table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS ai_messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                conversation_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (conversation_id) REFERENCES ai_conversations(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    /// Execute a raw SQL query (for custom operations)
    pub async fn execute(&self, query: &str) -> AppResult<u64> {
        let result = sqlx::query(query)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result.rows_affected())
    }

    /// Get the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Record file access
    pub async fn record_file_access(&self, file_id: i64) -> AppResult<()> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query(
            r#"
            UPDATE file_index 
            SET access_count = access_count + 1, last_accessed_at = ?
            WHERE id = ?
            "#,
        )
        .bind(now)
        .bind(file_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    /// Add clipboard history entry
    pub async fn add_clipboard_entry(
        &self,
        content_type: &str,
        content: &str,
        preview: Option<&str>,
        source_app: Option<&str>,
    ) -> AppResult<i64> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            r#"
            INSERT INTO clipboard_history (content_type, content, preview, source_app, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(content_type)
        .bind(content)
        .bind(preview)
        .bind(source_app)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result.last_insert_rowid())
    }

    /// Get recent clipboard history
    pub async fn get_clipboard_history(&self, limit: i64) -> AppResult<Vec<ClipboardEntry>> {
        let entries = sqlx::query_as::<_, ClipboardEntry>(
            r#"
            SELECT id, content_type, content, preview, source_app, is_favorite, created_at
            FROM clipboard_history
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(entries)
    }

    /// Record app launch
    pub async fn record_app_launch(&self, app_path: &str, app_name: &str) -> AppResult<()> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query(
            r#"
            INSERT INTO app_usage (app_path, app_name, launch_count, last_launched_at)
            VALUES (?, ?, 1, ?)
            ON CONFLICT(app_path) DO UPDATE SET
                launch_count = launch_count + 1,
                last_launched_at = ?
            "#,
        )
        .bind(app_path)
        .bind(app_name)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    /// Add search history entry
    pub async fn add_search_history(
        &self,
        query: &str,
        result_type: Option<&str>,
        result_id: Option<&str>,
    ) -> AppResult<()> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query(
            r#"
            INSERT INTO search_history (query, result_type, result_id, searched_at)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(query)
        .bind(result_type)
        .bind(result_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

/// Clipboard history entry
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ClipboardEntry {
    pub id: i64,
    pub content_type: String,
    pub content: String,
    pub preview: Option<String>,
    pub source_app: Option<String>,
    pub is_favorite: i64,
    pub created_at: i64,
}
