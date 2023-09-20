use async_graphql::ErrorExtensions;
use mongodb::bson::{self, Uuid};
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CVDataSourceError {
    // uuid cannot be found
    IdNotFound(bson::oid::ObjectId),

    // author id cannot be found
    AuthorIdNotFound(bson::oid::ObjectId),

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
    InvalidId(bson::oid::ObjectId),
}

impl fmt::Display for CVDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CVDataSourceError::IdNotFound(uuid) => {
                write!(f, "Id {:?} is not found", uuid)
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
            CVDataSourceError::InvalidId(id) => {
                write!(f, "Id {:?} is invalid", id)
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

#[cfg(test)]
mod tests {
    use mongodb::bson::{self, oid::ObjectId};

    use super::super::cv_data_source_error::CVDataSourceError;
    #[test]
    fn test_cv_id_not_found() {
        let uuid = bson::oid::ObjectId::new();
        let err = CVDataSourceError::IdNotFound(uuid);
        assert_eq!(format!("{}", err), format!("Id {:?} is not found", uuid));
    }

    #[test]
    fn test_too_long_description() {
        let err = CVDataSourceError::TooLongDescription;
        assert_eq!(format!("{}", err), format!("Description is too long"));
    }

    #[test]
    fn test_empty_title() {
        let err = CVDataSourceError::EmptyTitle;
        assert_eq!(format!("{}", err), format!("Title cannot be empty"));
    }

    #[test]
    fn test_empty_id() {
        let err = CVDataSourceError::EmptyId;
        assert_eq!(format!("{}", err), format!("Id cannot be empty"));
    }

    #[test]
    fn test_invalid_title() {
        let s = "hello".to_string();
        let err = CVDataSourceError::InvalidTitle(s.clone());
        assert_eq!(
            format!("{}", err),
            format!("Title {:?} is invalid", s.clone())
        );
    }

    #[test]
    fn test_invalid_id() {
        let id = ObjectId::new();
        let err = CVDataSourceError::InvalidId(id);
        assert_eq!(format!("{}", err), format!("Id {:?} is invalid", id));
    }

    #[test]
    fn test_too_long_title() {
        let err = CVDataSourceError::TooLongTitle;
        assert_eq!(format!("{}", err), format!("Title is too long"));
    }
}
