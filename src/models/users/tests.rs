use mongodb::bson::{self, oid::ObjectId, Uuid};

use crate::models::{
    education::Education,
    users::{create_user_input::CreateUserInputBuilder, update_user_input::UpdateUserInputBuilder}, sex::Sex,
};

use super::{CreateUserInput, User};

fn create_demo_user_input(_test_id: ObjectId) -> CreateUserInput {
    let dummy_uuid = Uuid::new();
    CreateUserInputBuilder::default()
        .with_username("username")
        .with_password("password")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_sex(Sex::Male)
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            school: "school 1".to_string(),
            major: "major 1".to_string(),
            minor: Some("minor 1".to_string()),
            degree: "degree 1".to_string(),
            start_date: None,
            end_date: None
        })
        .with_education(Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None
        })
        .with_about("about".to_string())
        .with_avatar(dummy_uuid)
        .with_cover_photo(dummy_uuid)
        .build()
        .unwrap()
}

fn create_demo_user(test_user_input: CreateUserInput) -> User {
    User::from(test_user_input)
}

#[test]
fn test_create_user_input_to_user() {
    let uuid = bson::oid::ObjectId::new();
    let test_user_input = create_demo_user_input(uuid);

    let _user = create_demo_user(test_user_input);
}

#[test]
fn test_update_user() {
    let id = bson::oid::ObjectId::new();
    let test_user_input = create_demo_user_input(id);

    let _user = create_demo_user(test_user_input);

    let update_user_input = UpdateUserInputBuilder::default()
        .with_user_id(id)
        .with_about("Updated about")
        .build()
        .unwrap();
    let _updated_user = User {
        id: update_user_input.user_id.into(),
        about: update_user_input.about,
        .._user
    };
}

#[test]
fn test_user_from_input() {
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_user_input = CreateUserInputBuilder::default()
        .with_username("username")
        .with_password("password")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_sex(Sex::Male)
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            school: "school 1".to_string(),
            major: "major 1".to_string(),
            minor: Some("minor 1".to_string()),
            degree: "degree 1".to_string(),
            start_date: None,
            end_date: None
        })
        .with_education(Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None
        })
        .with_about("about".to_string())
        .with_avatar(uuid)
        .with_cover_photo(uuid)
        .build()
        .unwrap();
    let user = User::from(test_user_input);
    assert_eq!(user.username, "username".to_string());
    assert_eq!(user.password, "password".to_string());
    assert_eq!(user.first_name, "first_name".to_string());
    assert_eq!(user.last_name, "last_name".to_string());
    assert_eq!(user.country, Some("country".to_string()));
    assert_eq!(user.skills, vec!["skill".to_string()]);
    assert_eq!(user.primary_email, "primary_email".to_string());
    assert_eq!(
        user.other_emails,
        vec!["other_mails".to_string(), "other_mails2".to_string()]
    );
    assert_eq!(user.about, Some("about".to_string()));
    assert_eq!(user.avatar, Some(uuid));
    assert_eq!(user.cover_photo, Some(uuid));
    assert_eq!(user.educations.len(), 2);
    assert_eq!(user.rating, None);
    assert_eq!(user.level, None);
    assert_eq!(user.shared_cvs, Vec::default());
    assert_eq!(user.saved_cvs, Vec::default());
    assert_eq!(user.liked_cvs, Vec::default());
    assert_eq!(user.friends_list, Vec::default());
    assert_eq!(user.cv, Vec::default());
}
