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
