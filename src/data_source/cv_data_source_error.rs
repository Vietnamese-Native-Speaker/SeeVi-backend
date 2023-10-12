
use mongodb::bson::oid::ObjectId;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CVDataSourceError {
    // uuid cannot be found
    ObjectIdNotFound(ObjectId),

    // author id cannot be found
    AuthorIdNotFound(ObjectId),

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
    InvalidId(ObjectId),

    // Cannot find CV
    QueryFail,
}


impl fmt::Display for CVDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CVDataSourceError::ObjectIdNotFound(uuid) => {
                write!(f, "ObjectId {:?} is not found", uuid)
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
            CVDataSourceError::InvalidId(objectid) => {
                write!(f, "ObjectId {:?} is invalid", objectid)
            }
            CVDataSourceError::TooLongTitle => {
                write!(f, "Title is too long")
            }
            CVDataSourceError::AuthorIdNotFound(objectid) => {
                write!(f, "Author id {:?} is not found", objectid)
            }
            CVDataSourceError::QueryFail => {
                write!(f, "Fail to find CV")
            }
        }
    }
}

