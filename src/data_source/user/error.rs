use async_graphql::ErrorExtensions;
use mongodb::bson;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserDataSourceError {
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

    // Database error
    DatabaseError,
}

impl fmt::Display for UserDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Display message for Uuid not found
            UserDataSourceError::IdNotFound(id) => {
                write!(f, "Id {:?} not found", id)
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

            UserDataSourceError::DatabaseError => {
                write!(f, "Database error")
            }
        }
    }
}

impl ErrorExtensions for UserDataSourceError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
            .extend_with(|_, e| e.set("code", "INVALID_USER"))
    }
}

#[cfg(test)]
mod tests {
    use mongodb::bson::{self, Uuid};

    use crate::data_source::mongo::{MongoDB, MongoForTesting};
    use crate::models::users::CreateUserInput;

    use super::super::{UserDataSource, UserDataSourceError};

    #[tokio::test]
    async fn basic_user_create_then_get() {
        let new_user_input = CreateUserInput::builder()
            .with_password("password")
            .with_last_name("LastName")
            .with_first_name("FirstName")
            .with_skill("Nothing")
            .with_about("Nothing")
            .with_country("VN")
            .with_primary_email("pemail")
            .with_username("username")
            .build()
            .unwrap();

        let db = MongoForTesting::init().await;
        db.create_user(new_user_input).await.unwrap();
        let user = db.get_user_by_username("username").await.unwrap();
        assert_eq!(user.username, "username");
    }

    #[test]
    fn test_user_id_not_found() {
        let uuid = bson::oid::ObjectId::new();
        let err = UserDataSourceError::IdNotFound(uuid);
        assert_eq!(format!("{}", err), format!("Id {:?} not found", uuid));
    }

    #[test]
    fn test_username_is_taken() {
        let username = String::from("username");
        let err = UserDataSourceError::UsernameTaken(username.clone());
        assert_eq!(
            format!("{}", err),
            format!("Username {:?} already taken", username)
        );
    }

    #[test]
    fn test_username_is_not_found() {
        let username = String::from("username");
        let err = UserDataSourceError::UsernameNotFound(username.clone());
        assert_eq!(
            format!("{}", err),
            format!("Username {:?} not found", username)
        );
    }

    #[test]
    fn test_email_is_taken() {
        let email = String::from("email");
        let err = UserDataSourceError::EmailTaken(email.clone());
        assert_eq!(
            format!("{}", err),
            format!("Email {:?} already taken", email)
        );
    }

    #[test]
    fn test_email_is_not_found() {
        let email = String::from("email");
        let err = UserDataSourceError::EmailNotFound(email.clone());
        assert_eq!(format!("{}", err), format!("Email {:?} not found", email));
    }

    #[test]
    fn test_email_invalid() {
        let email1 = String::from("email");
        let err1 = UserDataSourceError::InvalidEmail(email1.clone());

        let email2 = String::from("");
        let err2 = UserDataSourceError::InvalidEmail(email2.clone());

        assert_eq!(
            format!("{}", err1),
            format!("Email {:?} is invalid", email1)
        );
        assert_eq!(format!("{}", err2), format!("Email cannot be empty"));
    }

    #[test]
    fn test_username_invalid() {
        let username1 = String::from("username");
        let err1 = UserDataSourceError::InvalidUsername(username1.clone());

        let username2 = String::from("");
        let err2 = UserDataSourceError::InvalidUsername(username2.clone());

        assert_eq!(
            format!("{}", err1),
            format!("Username {:?} is invalid", username1)
        );
        assert_eq!(format!("{}", err2), format!("Username cannot be empty"));
    }

    #[test]
    fn test_name_invalid() {
        let name1 = String::from("name");
        let err1 = UserDataSourceError::InvalidNameField(name1.clone());

        let name2 = String::from("");
        let err2 = UserDataSourceError::InvalidNameField(name2.clone());

        assert_eq!(format!("{}", err1), format!("Name {:?} is invalid", name1));
        assert_eq!(format!("{}", err2), format!("Name cannot be empty"));
    }
}

#[test]
fn test_user_create_fail() {
    let err = UserDataSourceError::CreateUserFailed;
    assert_eq!(format!("{}", err), format!("Create user failed"));
}

#[test]
fn test_wrong_email_username_or_password() {
    let err = UserDataSourceError::WrongEmailUsernameOrPassword;
    assert_eq!(
        format!("{}", err),
        format!("Wrong email/username or password")
    );
}
