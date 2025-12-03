use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Parser error: {0}")]
    Parser(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Clipboard error: {0}")]
    Clipboard(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<tauri_plugin_clipboard_manager::Error> for AppError {
    fn from(err: tauri_plugin_clipboard_manager::Error) -> Self {
        AppError::Clipboard(err.to_string())
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type AppResult<T> = Result<T, AppError>;
