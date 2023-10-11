use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CVDetailsDataSourceError{
    // cannot find cv
    CVNotFound,

    // cannot find user
    UserNotFound,

    // query failed
    QueryError,

}

impl fmt::Display for CVDetailsDataSourceError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CVDetailsDataSourceError::UserNotFound => {
                write!(f, "cannot find user!")
            },
            CVDetailsDataSourceError::CVNotFound => {
                write!(f, "cannot find cv!")
            }
            CVDetailsDataSourceError::QueryError => {
                write!(f, "failed to do query!")
            }
        }
    }
}