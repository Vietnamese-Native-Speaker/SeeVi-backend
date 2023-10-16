use mongodb::bson;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]

pub enum LikeDataSourceError {
    /// Database error
    DatabaseError,
}

impl fmt::Display for LikeDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LikeDataSourceError::DatabaseError => write!(f, "Database error"),
        }
    }
}
