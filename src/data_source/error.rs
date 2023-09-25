use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum Error {
    NotFound,
    InternalServerError(mongodb::error::Error),
    Custom {
        message: String,
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl Error {
    pub fn custom(
        message: impl Into<String>,
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::Custom {
            message: message.into(),
            cause,
        }
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(error: mongodb::error::Error) -> Self {
        Self::InternalServerError(error)
    }
}
