use serde::{ser::SerializeStruct, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{message}")]
    Other {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self::Other {
            message: message.into(),
            source: None,
        }
    }

    fn kind(&self) -> &'static str {
        match self {
            Self::Io(_) => "io",
            Self::Network(_) => "network",
            Self::Json(_) => "json",
            Self::Other { .. } => "other",
        }
    }
}

impl From<String> for AppError {
    fn from(message: String) -> Self {
        Self::msg(message)
    }
}

impl From<&str> for AppError {
    fn from(message: &str) -> Self {
        Self::msg(message)
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut message = self.to_string();
        let mut src = std::error::Error::source(self);
        while let Some(e) = src {
            message.push_str(": ");
            message.push_str(&e.to_string());
            src = e.source();
        }

        let mut s = serializer.serialize_struct("AppError", 2)?;
        s.serialize_field("kind", self.kind())?;
        s.serialize_field("message", &message)?;
        s.end()
    }
}

pub trait Context<T> {
    fn context(self, msg: impl Into<String>) -> AppResult<T>;
    fn with_context<F, S>(self, f: F) -> AppResult<T>
    where
        F: FnOnce() -> S,
        S: Into<String>;
}

impl<T, E> Context<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, msg: impl Into<String>) -> AppResult<T> {
        self.map_err(|e| AppError::Other {
            message: msg.into(),
            source: Some(Box::new(e)),
        })
    }

    fn with_context<F, S>(self, f: F) -> AppResult<T>
    where
        F: FnOnce() -> S,
        S: Into<String>,
    {
        self.map_err(|e| AppError::Other {
            message: f().into(),
            source: Some(Box::new(e)),
        })
    }
}
