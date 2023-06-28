use mongodb::bson::Uuid;

use crate::models::{
    education::Education,
    users::{create_user_input::CreateUserInputBuilder, update_user_input::UpdateUserInputBuilder},
};

use super::User;

#[test]
fn test_create_user_input_to_user() {
    let uuid = Uuid::new();
    let test_user_input = CreateUserInputBuilder::default()
        .with_username("username")
        .with_password("password")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_education(Education {
            institution: "University of Example 2".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_about("about".to_string())
        .with_avatar(uuid)
        .with_cover_photo(uuid)
        .build()
        .unwrap();

    let _user = User {
        user_id: Uuid::new(),
        username: test_user_input.username,
        password: test_user_input.password,
        first_name: test_user_input.first_name,
        last_name: test_user_input.last_name,
        country: test_user_input.country,
        skills: test_user_input.skills,
        primary_email: test_user_input.primary_email,
        other_mails: test_user_input.other_mails,
        about: test_user_input.about,
        education: test_user_input.education,
        avatar: test_user_input.avatar,
        cover_photo: test_user_input.cover_photo,
        rating: test_user_input.rating,
        level: test_user_input.level,
        shared_cvs: Vec::default(),
        saved_cvs: Vec::default(),
        liked_cvs: Vec::default(),
        friends_list: Vec::default(),
        cv: Vec::default(),
    };
}

#[test]
fn test_update_user() {
    let uuid = Uuid::new();
    let user_uuid = Uuid::new();
    let test_user_input = CreateUserInputBuilder::default()
        .with_username("username")
        .with_password("password")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_education(Education {
            institution: "University of Example 2".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_about("about".to_string())
        .with_avatar(uuid)
        .with_cover_photo(uuid)
        .build()
        .unwrap();

    let _user = User {
        user_id: user_uuid,
        username: test_user_input.username,
        password: test_user_input.password,
        first_name: test_user_input.first_name,
        last_name: test_user_input.last_name,
        country: test_user_input.country,
        skills: test_user_input.skills,
        primary_email: test_user_input.primary_email,
        other_mails: test_user_input.other_mails,
        about: test_user_input.about,
        education: test_user_input.education,
        avatar: test_user_input.avatar,
        cover_photo: test_user_input.cover_photo,
        rating: test_user_input.rating,
        level: test_user_input.level,
        shared_cvs: Vec::default(),
        saved_cvs: Vec::default(),
        liked_cvs: Vec::default(),
        friends_list: Vec::default(),
        cv: Vec::default(),
    };

    let update_user_input = UpdateUserInputBuilder::default()
        .with_user_id(uuid)
        .with_about("Updated about")
        .build()
        .unwrap();
    let _updated_user = User {
        user_id: update_user_input.user_id,
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
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_education(Education {
            institution: "University of Example 2".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
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
    assert_eq!(user.other_mails, vec!["other_mails".to_string(), "other_mails2".to_string()]);
    assert_eq!(user.about, Some("about".to_string()));
    assert_eq!(user.avatar, Some(uuid));
    assert_eq!(user.cover_photo, Some(uuid));
    assert_eq!(user.education.len(), 2);
    assert_eq!(user.rating, None);
    assert_eq!(user.level, None);
    assert_eq!(user.shared_cvs, Vec::default());
    assert_eq!(user.saved_cvs, Vec::default());
    assert_eq!(user.liked_cvs, Vec::default());
    assert_eq!(user.friends_list, Vec::default());
    assert_eq!(user.cv, Vec::default());
}