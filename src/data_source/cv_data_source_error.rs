use async_graphql::ErrorExtensions;
use mongodb::bson::Uuid;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CVDataSourceError {
    // uuid cannot be found
    UuidNotFound(Uuid),

    // author id cannot be found
    AuthorIdNotFound(Uuid),

    // description is longer than limit
    TooLongDescription,

    // title is empty
    EmptyTitle,

    // id is empty
    EmptyId,

    // title is too long
    TooLongTitle,

    // title is invalid
    InvalidTitle(String),

    // id is invalid
    InvalidId(Uuid),
}

impl fmt::Display for CVDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CVDataSourceError::UuidNotFound(uuid) => {
                write!(f, "Uuid {:?} is not found", uuid)
            }
            CVDataSourceError::TooLongDescription => {
                write!(f, "Description is too long")
            }
            CVDataSourceError::EmptyTitle => {
                write!(f, "Title cannot be empty")
            }
            CVDataSourceError::EmptyId => {
                write!(f, "Id cannot be empty")
            }
            CVDataSourceError::InvalidTitle(s) => {
                write!(f, "Title {:?} is invalid", s)
            }
            CVDataSourceError::InvalidId(uuid) => {
                write!(f, "Uuid {:?} is invalid", uuid)
            }
            CVDataSourceError::TooLongTitle => {
                write!(f, "Title is too long")
            }
            CVDataSourceError::AuthorIdNotFound(uuid) => {
                write!(f, "Author id {:?} is not found", uuid)
            }
        }
    }
}

impl ErrorExtensions for CVDataSourceError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string()).extend_with(|_, e| e.set("code", "INVALID_CV"))
    }
}
