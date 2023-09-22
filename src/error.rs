use async_graphql::ErrorExtensions;
use warp::reject::Reject;

/// Error type for server errors.
/// This type is use for both warp rejections and async_graphql errors.
#[derive(Debug)]
pub enum ServerError {
    NotFound,
    Unauthorized,
    InvalidToken,
    InternalServerError,
    InvalidAuthorizationHeader,
}

impl ToString for ServerError {
    fn to_string(&self) -> String {
        match self {
            ServerError::NotFound => "Not found".to_string(),
            ServerError::Unauthorized => "Unauthorized".to_string(),
            ServerError::InternalServerError => "Internal server error".to_string(),
            ServerError::InvalidToken => "Invalid token".to_string(),
            ServerError::InvalidAuthorizationHeader => "Invalid authorization header".to_string(),
        }
    }
}

impl ErrorExtensions for ServerError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string()).extend_with(|_, e| match self {
            ServerError::NotFound => e.set("code", "NOT_FOUND"),
            ServerError::Unauthorized => e.set("code", "UNAUTHORIZED"),
            ServerError::InternalServerError => e.set("code", "INTERNAL_SERVER_ERROR"),
            ServerError::InvalidToken => e.set("code", "INVALID_TOKEN"),
            ServerError::InvalidAuthorizationHeader => unreachable!(),
        })
    }
}

impl Reject for ServerError {}
