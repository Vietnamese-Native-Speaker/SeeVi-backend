use mongodb::bson::Uuid;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
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

    /// Error when the user name is empty.
    EmptyUsername,

    /// Error when the email is empty.
    EmptyEmail,

    /// Error when the name is empty.
    EmptyName,

    /// Error when the username is invalid.
    InvalidUsername(String),

    /// Error when the email is invalid.
    InvalidEmail(String),

    /// Error when the name field provided is invalid.
    InvalidNameField(String),

    /// Error when create user fails.
    CreateUserFailed,

    /// Error when wrong username or password is provided.
    WrongEmailUsernameOrPassword,

    // Error when password is invalid
    InvalidPassword,

    // Error when update user fails
    UpdateUserFailed,

    // Token is invalid
    InvalidToken,
}

impl fmt::Display for UserDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Display message for Uuid not found
            UserDataSourceError::UuidNotFound(uuid) => {
                write!(f, "Uuid {:?} not found", uuid)
            }

            // Display message for username not found
            UserDataSourceError::UsernameNotFound(username) => {
                write!(f, "Username {:?} not found", username)
            }

            // Display message for username taken
            UserDataSourceError::UsernameTaken(username) => {
                write!(f, "Username {:?} already taken", username)
            }

            // Display message for email not found
            UserDataSourceError::EmailNotFound(email) => {
                write!(f, "Email {:?} not found", email)
            }

            // Display message for email taken
            UserDataSourceError::EmailTaken(email) => {
                write!(f, "Email {:?} already taken", email)
            }

            // Display message for empty username
            UserDataSourceError::EmptyUsername => {
                write!(f, "Username cannot be empty")
            }

            // Display message for empty email
            UserDataSourceError::EmptyEmail => {
                write!(f, "Email cannot be empty")
            }

            // Display message for empty name
            UserDataSourceError::EmptyName => {
                write!(f, "Name cannot be empty")
            }

            // Display message for invalid username
            UserDataSourceError::InvalidUsername(username) => {
                if username.len() == 0 {
                    UserDataSourceError::EmptyUsername.fmt(f)
                } else {
                    write!(f, "Username {:?} is invalid", username)
                }
            }

            // Display message for invalid email
            UserDataSourceError::InvalidEmail(email) => {
                if email.len() == 0 {
                    UserDataSourceError::EmptyEmail.fmt(f)
                } else {
                    write!(f, "Email {:?} is invalid", email)
                }
            }

            // Display message for invalid name
            UserDataSourceError::InvalidNameField(name) => {
                if name.len() == 0 {
                    UserDataSourceError::EmptyName.fmt(f)
                } else {
                    write!(f, "Name {:?} is invalid", name)
                }
            }

            // Display message for create user failed
            UserDataSourceError::CreateUserFailed => {
                write!(f, "Create user failed")
            }

            // Display message for wrong email/username or password
            UserDataSourceError::WrongEmailUsernameOrPassword => {
                write!(f, "Wrong email/username or password")
            }

            UserDataSourceError::InvalidPassword => {
                write!(f, "Password is invalid")
            }

            UserDataSourceError::UpdateUserFailed => {
                write!(f, "Update user failed")
            }

            UserDataSourceError::InvalidToken => {
                write!(f, "Token is invalid")
            }
        }
    }
}
