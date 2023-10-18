use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BookmarkDataSourceError {
    // fail to do queries
    QueryFail,
}

impl fmt::Display for BookmarkDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookmarkDataSourceError::QueryFail => {
                write!(f, "fail to do queries")
            }
        }
    }
}
