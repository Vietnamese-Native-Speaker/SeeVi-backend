use std::fmt;
use mongodb::bson::Uuid;

pub enum UserDataSourceError {
    /// Error when the uid is not found
    UuidNotFound(Uuid),
    
    /// Error when the username is not found.
    UsernameNotFound(String),

    /// Error when the username is already taken.
    UsernameTaken(String),

    /// Error when the email is not found.
    EmailNotFound(String),

    /// Error when the email is already taken.
    EmailTaken(String),
}

impl fmt::Display for UserDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Display message for Uuid not found
            UserDataSourceError::UuidNotFound(uuid) => {
                write!(f, "Uuid {:?} not found", uuid)
            },

            // Display message for username not found
            UserDataSourceError::UsernameNotFound(username) => {
                write!(f, "Username {:?} not found", username)
            },

            // Display message for username taken
            UserDataSourceError::UsernameTaken(username) => {
                write!(f, "Username {:?} already taken", username)
            },

            // Display message for email not found
            UserDataSourceError::EmailNotFound(email) => {
                write!(f, "Email {:?} not found", email)
            },

            // Display message for email taken
            UserDataSourceError::EmailTaken(email) => {
                write!(f, "Email {:?} already taken", email)
            },
        }
    }
}
