mod cv_tests {
    use super::super::cv_data_source_error::CVDataSourceError;
    #[test]
    fn test_cv_id_not_found() {
        use mongodb::bson::Uuid;

        let uuid = Uuid::new();
        let err = CVDataSourceError::UuidNotFound(uuid);
        assert_eq!(format!("{}", err), format!("Uuid {:?} is not found", uuid));
    }

    #[test]
    fn test_too_long_description() {
        let err = CVDataSourceError::TooLongDescription;
        assert_eq!(format!("{}", err), format!("Description is too long"));
    }

    #[test]
    fn test_empty_title() {
        let err = CVDataSourceError::EmptyTitle;
        assert_eq!(format!("{}", err), format!("Title cannot be empty"));
    }

    #[test]
    fn test_empty_id() {
        let err = CVDataSourceError::EmptyId;
        assert_eq!(format!("{}", err), format!("Id cannot be empty"));
    }

    #[test]
    fn test_invalid_title() {
        let s = "hello".to_string();
        let err = CVDataSourceError::InvalidTitle(s.clone());
        assert_eq!(
            format!("{}", err),
            format!("Title {:?} is invalid", s.clone())
        );
    }

    #[test]
    fn test_invalid_id() {
        use mongodb::bson::Uuid;
        let uuid = Uuid::new();
        let err = CVDataSourceError::InvalidId(uuid);
        assert_eq!(format!("{}", err), format!("Uuid {:?} is invalid", uuid));
    }

    #[test]
    fn test_too_long_title() {
        let err = CVDataSourceError::TooLongTitle;
        assert_eq!(format!("{}", err), format!("Title is too long"));
    }
}

mod user_tests {
    use mongodb::bson::Uuid;

    use crate::data_source::mongo::MongoDB;
    use crate::models::users::CreateUserInput;

    use super::super::cv_data_source::CVDataSource;
    use super::super::user_data_source::UserDataSource;
    use super::super::user_data_source_error::UserDataSourceError;

    #[tokio::test]
    async fn basic_user_create_then_get() {
        let user_uuid = Uuid::new();
        let new_user_input = CreateUserInput::builder()
            .with_last_name("LastName")
            .with_first_name("FirstName")
            .with_skill("Nothing")
            .with_about("Nothing")
            .with_country("VN")
            .with_primary_email("pemail")
            .with_username("username")
            .build()
            .unwrap();

        let db = MongoDB::init_test().await;
        db.create_user(new_user_input).await.unwrap();
        let user = db.get_user_by_username("username").await.unwrap();
        assert_eq!(user.username, "username");
    }

    #[test]
    fn test_user_id_not_found() {
        use mongodb::bson::Uuid;

        let uuid = Uuid::new();
        let err = UserDataSourceError::UuidNotFound(uuid);
        assert_eq!(format!("{}", err), format!("Uuid {:?} not found", uuid));
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
