use async_graphql::ErrorExtensions;
use mongodb::bson;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]

pub enum UserServiceError {
    /// Error when the uid is not found
    IdNotFound(bson::oid::ObjectId),

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

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Display message for Uuid not found
            UserServiceError::IdNotFound(id) => {
                write!(f, "Id {:?} not found", id)
            }

            // Display message for username not found
            UserServiceError::UsernameNotFound(username) => {
                write!(f, "Username {:?} not found", username)
            }

            // Display message for username taken
            UserServiceError::UsernameTaken(username) => {
                write!(f, "Username {:?} already taken", username)
            }

            // Display message for email not found
            UserServiceError::EmailNotFound(email) => {
                write!(f, "Email {:?} not found", email)
            }

            // Display message for email taken
            UserServiceError::EmailTaken(email) => {
                write!(f, "Email {:?} already taken", email)
            }

            // Display message for empty username
            UserServiceError::EmptyUsername => {
                write!(f, "Username cannot be empty")
            }

            // Display message for empty email
            UserServiceError::EmptyEmail => {
                write!(f, "Email cannot be empty")
            }

            // Display message for empty name
            UserServiceError::EmptyName => {
                write!(f, "Name cannot be empty")
            }

            // Display message for invalid username
            UserServiceError::InvalidUsername(username) => {
                if username.len() == 0 {
                    UserServiceError::EmptyUsername.fmt(f)
                } else {
                    write!(f, "Username {:?} is invalid", username)
                }
            }

            // Display message for invalid email
            UserServiceError::InvalidEmail(email) => {
                if email.len() == 0 {
                    UserServiceError::EmptyEmail.fmt(f)
                } else {
                    write!(f, "Email {:?} is invalid", email)
                }
            }

            // Display message for invalid name
            UserServiceError::InvalidNameField(name) => {
                if name.len() == 0 {
                    UserServiceError::EmptyName.fmt(f)
                } else {
                    write!(f, "Name {:?} is invalid", name)
                }
            }

            // Display message for create user failed
            UserServiceError::CreateUserFailed => {
                write!(f, "Create user failed")
            }

            // Display message for wrong email/username or password
            UserServiceError::WrongEmailUsernameOrPassword => {
                write!(f, "Wrong email/username or password")
            }

            UserServiceError::InvalidPassword => {
                write!(f, "Password is invalid")
            }

            UserServiceError::UpdateUserFailed => {
                write!(f, "Update user failed")
            }

            UserServiceError::InvalidToken => {
                write!(f, "Token is invalid")
            }
        }
    }
}

impl ErrorExtensions for UserServiceError {
    fn extend(&self) -> async_graphql::Error {
        let code = match self {
            UserServiceError::IdNotFound(_) => "ID_NOT_FOUND",
            UserServiceError::UsernameNotFound(_) => "USERNAME_NOT_FOUND",
            UserServiceError::UsernameTaken(_) => "USERNAME_TAKEN",
            UserServiceError::EmailNotFound(_) => "EMAIL_NOT_FOUND",
            UserServiceError::EmailTaken(_) => "EMAIL_TAKEN",
            UserServiceError::EmptyUsername => "EMPTY_USERNAME",
            UserServiceError::EmptyEmail => "EMPTY_EMAIL",
            UserServiceError::EmptyName => "EMPTY_NAME",
            UserServiceError::InvalidUsername(_) => "INVALID_USERNAME",
            UserServiceError::InvalidEmail(_) => "INVALID_EMAIL",
            UserServiceError::InvalidNameField(_) => "INVALID_NAME_FIELD",
            UserServiceError::CreateUserFailed => "CREATE_USER_FAILED",
            UserServiceError::WrongEmailUsernameOrPassword => "WRONG_EMAIL_USERNAME_PASSWORD",
            UserServiceError::InvalidPassword => "INVALID_PASSWORD",
            UserServiceError::UpdateUserFailed => "UPDATE_USER_FAILED",
            UserServiceError::InvalidToken => "INVALID_TOKEN",
        };
        async_graphql::Error::new(self.to_string()).extend_with(|_, e| e.set("code", code))
    }
}
