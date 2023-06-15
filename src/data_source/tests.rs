#[test]
fn test_user_id_not_found() {
    use mongodb::bson::Uuid;
    use super::user_data_source_error::UserDataSourceError;

    let uuid = Uuid::new();
    let err = UserDataSourceError::UuidNotFound(uuid);
    assert_eq!(format!("{}", err), format!("Uuid {:?} not found", uuid));
}

#[test]
fn test_username_is_taken() {
    use super::user_data_source_error::UserDataSourceError;

    let username = String::from("username");
    let err = UserDataSourceError::UsernameTaken(username.clone());
    assert_eq!(format!("{}", err), format!("Username {:?} already taken", username));
}

#[test]
fn test_username_is_not_found() {
    use super::user_data_source_error::UserDataSourceError;

    let username = String::from("username");
    let err = UserDataSourceError::UsernameNotFound(username.clone());
    assert_eq!(format!("{}", err), format!("Username {:?} not found", username));
}

#[test]
fn test_email_is_taken() {
    use super::user_data_source_error::UserDataSourceError;

    let email = String::from("email");
    let err = UserDataSourceError::EmailTaken(email.clone());
    assert_eq!(format!("{}", err), format!("Email {:?} already taken", email));
}

#[test]
fn test_email_is_not_found() {
    use super::user_data_source_error::UserDataSourceError;

    let email = String::from("email");
    let err = UserDataSourceError::EmailNotFound(email.clone());
    assert_eq!(format!("{}", err), format!("Email {:?} not found", email));
}

#[test]
fn test_email_invalid() {
    use super::user_data_source_error::UserDataSourceError;

    let email1 = String::from("email");
    let err1 = UserDataSourceError::InvalidEmail(email1.clone());

    let email2 = String::from("");
    let err2 = UserDataSourceError::InvalidEmail(email2.clone());

    assert_eq!(format!("{}", err1), format!("Email {:?} is invalid", email1));
    assert_eq!(format!("{}", err2), format!("Email cannot be empty"));
}

#[test]
fn test_username_invalid() {
    use super::user_data_source_error::UserDataSourceError;

    let username1 = String::from("username");
    let err1 = UserDataSourceError::InvalidUsername(username1.clone());

    let username2 = String::from("");
    let err2 = UserDataSourceError::InvalidUsername(username2.clone());

    assert_eq!(format!("{}", err1), format!("Username {:?} is invalid", username1));
    assert_eq!(format!("{}", err2), format!("Username cannot be empty"));
}

#[test]
fn test_name_invalid() {
    use super::user_data_source_error::UserDataSourceError;

    let name1 = String::from("name");
    let err1 = UserDataSourceError::InvalidNameField(name1.clone());

    let name2 = String::from("");
    let err2 = UserDataSourceError::InvalidNameField(name2.clone());

    assert_eq!(format!("{}", err1), format!("Name {:?} is invalid", name1));
    assert_eq!(format!("{}", err2), format!("Name cannot be empty"));
}

#[test]
fn test_user_create_fail() {
    use super::user_data_source_error::UserDataSourceError;

    let err = UserDataSourceError::CreateUserFailed;
    assert_eq!(format!("{}", err), format!("Create user failed"));
}

#[test]
fn test_wrong_email_username_or_password() {
    use super::user_data_source_error::UserDataSourceError;

    let err = UserDataSourceError::WrongEmailUsernameOrPassword;
    assert_eq!(format!("{}", err), format!("Wrong email/username or password"));
}